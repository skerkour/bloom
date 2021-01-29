use crate::read_u8;
use error::{Error, Result, UnsupportedFeature};
use huffman::{fill_default_mjpeg_tables, HuffmanDecoder, HuffmanTable};
use marker::Marker;
use parser::{AdobeColorTransform, AppData, CodingProcess, Component, Dimensions, EntropyCoding, FrameInfo,
             parse_app, parse_com, parse_dht, parse_dqt, parse_dri, parse_sof, parse_sos, IccChunk,
             ScanInfo};
use upsampler::Upsampler;
use std::cmp;
use std::io::Read;
use std::mem;
use std::ops::Range;
use std::sync::Arc;
use worker::{RowData, PlatformWorker, Worker};

pub const MAX_COMPONENTS: usize = 4;

static UNZIGZAG: [u8; 64] = [
     0,  1,  8, 16,  9,  2,  3, 10,
    17, 24, 32, 25, 18, 11,  4,  5,
    12, 19, 26, 33, 40, 48, 41, 34,
    27, 20, 13,  6,  7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36,
    29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46,
    53, 60, 61, 54, 47, 55, 62, 63,
];

/// An enumeration over combinations of color spaces and bit depths a pixel can have.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelFormat {
    /// Luminance (grayscale), 8 bits
    L8,
    /// RGB, 8 bits per channel
    RGB24,
    /// CMYK, 8 bits per channel
    CMYK32,
}

impl PixelFormat {
    /// Determine the size in bytes of each pixel in this format
    pub fn pixel_bytes(&self) -> usize {
        match self {
            PixelFormat::L8 => 1,
            PixelFormat::RGB24 => 3,
            PixelFormat::CMYK32 => 4,
        }
    }
}

/// Represents metadata of an image.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageInfo {
    /// The width of the image, in pixels.
    pub width: u16,
    /// The height of the image, in pixels.
    pub height: u16,
    /// The pixel format of the image.
    pub pixel_format: PixelFormat,
}

/// JPEG decoder
pub struct Decoder<R> {
    reader: R,

    frame: Option<FrameInfo>,
    dc_huffman_tables: Vec<Option<HuffmanTable>>,
    ac_huffman_tables: Vec<Option<HuffmanTable>>,
    quantization_tables: [Option<Arc<[u16; 64]>>; 4],

    restart_interval: u16,
    color_transform: Option<AdobeColorTransform>,
    is_jfif: bool,
    is_mjpeg: bool,

    icc_markers: Vec<IccChunk>,

    // Used for progressive JPEGs.
    coefficients: Vec<Vec<i16>>,
    // Bitmask of which coefficients has been completely decoded.
    coefficients_finished: [u64; MAX_COMPONENTS],
}

impl<R: Read> Decoder<R> {
    /// Creates a new `Decoder` using the reader `reader`.
    pub fn new(reader: R) -> Decoder<R> {
        Decoder {
            reader: reader,
            frame: None,
            dc_huffman_tables: vec![None, None, None, None],
            ac_huffman_tables: vec![None, None, None, None],
            quantization_tables: [None, None, None, None],
            restart_interval: 0,
            color_transform: None,
            is_jfif: false,
            is_mjpeg: false,
            icc_markers: Vec::new(),
            coefficients: Vec::new(),
            coefficients_finished: [0; MAX_COMPONENTS],
        }
    }

    /// Returns metadata about the image.
    ///
    /// The returned value will be `None` until a call to either `read_info` or `decode` has
    /// returned `Ok`.
    pub fn info(&self) -> Option<ImageInfo> {
        match self.frame {
            Some(ref frame) => {
                let pixel_format = match frame.components.len() {
                    1 => PixelFormat::L8,
                    3 => PixelFormat::RGB24,
                    4 => PixelFormat::CMYK32,
                    _ => panic!(),
                };

                Some(ImageInfo {
                    width: frame.output_size.width,
                    height: frame.output_size.height,
                    pixel_format: pixel_format,
                })
            },
            None => None,
        }
    }

    /// Returns the embeded icc profile if the image contains one.
    pub fn icc_profile(&self) -> Option<Vec<u8>> {
        let mut marker_present: [Option<&IccChunk>; 256] = [None; 256];
        let num_markers = self.icc_markers.len();
        if num_markers == 0 && num_markers < 256 {
            return None;
        }
        // check the validity of the markers
        for chunk in &self.icc_markers {
            if usize::from(chunk.num_markers) != num_markers {
                // all the lengths must match
                return None;
            }
            if chunk.seq_no == 0 {
                return None;
            }
            if marker_present[usize::from(chunk.seq_no)].is_some() {
                // duplicate seq_no
                return None;
            } else {
                marker_present[usize::from(chunk.seq_no)] = Some(chunk);
            }
        }

        // assemble them together by seq_no failing if any are missing
        let mut data = Vec::new();
        // seq_no's start at 1
        for &chunk in marker_present.get(1..=num_markers)? {
            data.extend_from_slice(&chunk?.data);
        }
        Some(data)
    }

    /// Tries to read metadata from the image without decoding it.
    ///
    /// If successful, the metadata can be obtained using the `info` method.
    pub fn read_info(&mut self) -> Result<()> {
        self.decode_internal(true).map(|_| ())
    }

    /// Configure the decoder to scale the image during decoding.
    ///
    /// This efficiently scales the image by the smallest supported scale
    /// factor that produces an image larger than or equal to the requested
    /// size in at least one axis. The currently implemented scale factors
    /// are 1/8, 1/4, 1/2 and 1.
    ///
    /// To generate a thumbnail of an exact size, pass the desired size and
    /// then scale to the final size using a traditional resampling algorithm.
    pub fn scale(&mut self, requested_width: u16, requested_height: u16) -> Result<(u16, u16)> {
        self.read_info()?;
        let frame = self.frame.as_mut().unwrap();
        let idct_size = crate::idct::choose_idct_size(frame.image_size, Dimensions{ width: requested_width, height: requested_height });
        frame.update_idct_size(idct_size)?;
        Ok((frame.output_size.width, frame.output_size.height))
    }

    /// Decodes the image and returns the decoded pixels if successful.
    pub fn decode(&mut self) -> Result<Vec<u8>> {
        self.decode_internal(false)
    }

    fn decode_internal(&mut self, stop_after_metadata: bool) -> Result<Vec<u8>> {
        if stop_after_metadata && self.frame.is_some() {
            // The metadata has already been read.
            return Ok(Vec::new());
        }
        else if self.frame.is_none() && (read_u8(&mut self.reader)? != 0xFF || Marker::from_u8(read_u8(&mut self.reader)?) != Some(Marker::SOI)) {
            return Err(Error::Format("first two bytes are not an SOI marker".to_owned()));
        }

        let mut previous_marker = Marker::SOI;
        let mut pending_marker = None;
        let mut worker = None;
        let mut scans_processed = 0;
        let mut planes = vec![Vec::new(); self.frame.as_ref().map_or(0, |frame| frame.components.len())];

        loop {
            let marker = match pending_marker.take() {
                Some(m) => m,
                None => self.read_marker()?,
            };

            match marker {
                // Frame header
                Marker::SOF(..) => {
                    // Section 4.10
                    // "An image contains only one frame in the cases of sequential and
                    //  progressive coding processes; an image contains multiple frames for the
                    //  hierarchical mode."
                    if self.frame.is_some() {
                        return Err(Error::Unsupported(UnsupportedFeature::Hierarchical));
                    }

                    let frame = parse_sof(&mut self.reader, marker)?;
                    let component_count = frame.components.len();

                    if frame.is_differential {
                        return Err(Error::Unsupported(UnsupportedFeature::Hierarchical));
                    }
                    if frame.coding_process == CodingProcess::Lossless {
                        return Err(Error::Unsupported(UnsupportedFeature::Lossless));
                    }
                    if frame.entropy_coding == EntropyCoding::Arithmetic {
                        return Err(Error::Unsupported(UnsupportedFeature::ArithmeticEntropyCoding));
                    }
                    if frame.precision != 8 {
                        return Err(Error::Unsupported(UnsupportedFeature::SamplePrecision(frame.precision)));
                    }
                    if component_count != 1 && component_count != 3 && component_count != 4 {
                        return Err(Error::Unsupported(UnsupportedFeature::ComponentCount(component_count as u8)));
                    }

                    // Make sure we support the subsampling ratios used.
                    let _ = Upsampler::new(&frame.components, frame.image_size.width, frame.image_size.height)?;

                    self.frame = Some(frame);

                    if stop_after_metadata {
                        return Ok(Vec::new());
                    }

                    planes = vec![Vec::new(); component_count];
                },

                // Scan header
                Marker::SOS => {
                    if self.frame.is_none() {
                        return Err(Error::Format("scan encountered before frame".to_owned()));
                    }
                    if worker.is_none() {
                        worker = Some(PlatformWorker::new()?);
                    }

                    let frame = self.frame.clone().unwrap();
                    let scan = parse_sos(&mut self.reader, &frame)?;

                    if frame.coding_process == CodingProcess::DctProgressive && self.coefficients.is_empty() {
                        self.coefficients = frame.components.iter().map(|c| {
                            let block_count = c.block_size.width as usize * c.block_size.height as usize;
                            vec![0; block_count * 64]
                        }).collect();
                    }

                    // This was previously buggy, so let's explain the log here a bit. When a
                    // progressive frame is encoded then the coefficients (DC, AC) of each
                    // component (=color plane) can be split amongst scans. In particular it can
                    // happen or at least occurs in the wild that a scan contains coefficient 0 of
                    // all components. If now one but not all components had all other coefficients
                    // delivered in previous scans then such a scan contains all components but
                    // completes only some of them! (This is technically NOT permitted for all
                    // other coefficients as the standard dictates that scans with coefficients
                    // other than the 0th must only contain ONE component so we would either
                    // complete it or not. We may want to detect and error in case more component
                    // are part of a scan than allowed.) What a weird edge case.
                    //
                    // But this means we track precisely which components get completed here.
                    let mut finished = [false; MAX_COMPONENTS];

                    if scan.successive_approximation_low == 0 {
                        for (&i, component_finished) in scan.component_indices.iter().zip(&mut finished) {
                            if self.coefficients_finished[i] == !0 {
                                continue;
                            }
                            for j in scan.spectral_selection.clone() {
                                self.coefficients_finished[i] |= 1 << j;
                            }
                            if self.coefficients_finished[i] == !0 {
                                *component_finished = true;
                            }
                        }
                    }

                    let (marker, data) = self.decode_scan(&frame, &scan, worker.as_mut().unwrap(), &finished)?;

                    if let Some(data) = data {
                        for (i, plane) in data.into_iter().enumerate().filter(|&(_, ref plane)| !plane.is_empty()) {
                            if self.coefficients_finished[i] == !0 {
                                planes[i] = plane;
                            }
                        }
                    }

                    pending_marker = marker;
                    scans_processed += 1;
                },

                // Table-specification and miscellaneous markers
                // Quantization table-specification
                Marker::DQT => {
                    let tables = parse_dqt(&mut self.reader)?;

                    for (i, &table) in tables.iter().enumerate() {
                        if let Some(table) = table {
                            let mut unzigzagged_table = [0u16; 64];

                            for j in 0 .. 64 {
                                unzigzagged_table[UNZIGZAG[j] as usize] = table[j];
                            }

                            self.quantization_tables[i] = Some(Arc::new(unzigzagged_table));
                        }
                    }
                },
                // Huffman table-specification
                Marker::DHT => {
                    let is_baseline = self.frame.as_ref().map(|frame| frame.is_baseline);
                    let (dc_tables, ac_tables) = parse_dht(&mut self.reader, is_baseline)?;

                    let current_dc_tables = mem::replace(&mut self.dc_huffman_tables, vec![]);
                    self.dc_huffman_tables = dc_tables.into_iter()
                                                      .zip(current_dc_tables.into_iter())
                                                      .map(|(a, b)| a.or(b))
                                                      .collect();

                    let current_ac_tables = mem::replace(&mut self.ac_huffman_tables, vec![]);
                    self.ac_huffman_tables = ac_tables.into_iter()
                                                      .zip(current_ac_tables.into_iter())
                                                      .map(|(a, b)| a.or(b))
                                                      .collect();
                },
                // Arithmetic conditioning table-specification
                Marker::DAC => return Err(Error::Unsupported(UnsupportedFeature::ArithmeticEntropyCoding)),
                // Restart interval definition
                Marker::DRI => self.restart_interval = parse_dri(&mut self.reader)?,
                // Comment
                Marker::COM => {
                    let _comment = parse_com(&mut self.reader)?;
                },
                // Application data
                Marker::APP(..) => {
                    if let Some(data) = parse_app(&mut self.reader, marker)? {
                        match data {
                            AppData::Adobe(color_transform) => self.color_transform = Some(color_transform),
                            AppData::Jfif => {
                                // From the JFIF spec:
                                // "The APP0 marker is used to identify a JPEG FIF file.
                                //     The JPEG FIF APP0 marker is mandatory right after the SOI marker."
                                // Some JPEGs in the wild does not follow this though, so we allow
                                // JFIF headers anywhere APP0 markers are allowed.
                                /*
                                if previous_marker != Marker::SOI {
                                    return Err(Error::Format("the JFIF APP0 marker must come right after the SOI marker".to_owned()));
                                }
                                */

                                self.is_jfif = true;
                            },
                            AppData::Avi1 => self.is_mjpeg = true,
                            AppData::Icc(icc) => self.icc_markers.push(icc),
                        }
                    }
                },
                // Restart
                Marker::RST(..) => {
                    // Some encoders emit a final RST marker after entropy-coded data, which
                    // decode_scan does not take care of. So if we encounter one, we ignore it.
                    if previous_marker != Marker::SOS {
                        return Err(Error::Format("RST found outside of entropy-coded data".to_owned()));
                    }
                },

                // Define number of lines
                Marker::DNL => {
                    // Section B.2.1
                    // "If a DNL segment (see B.2.5) is present, it shall immediately follow the first scan."
                    if previous_marker != Marker::SOS || scans_processed != 1 {
                        return Err(Error::Format("DNL is only allowed immediately after the first scan".to_owned()));
                    }

                    return Err(Error::Unsupported(UnsupportedFeature::DNL));
                },

                // Hierarchical mode markers
                Marker::DHP | Marker::EXP => return Err(Error::Unsupported(UnsupportedFeature::Hierarchical)),

                // End of image
                Marker::EOI => break,

                _ => return Err(Error::Format(format!("{:?} marker found where not allowed", marker))),
            }

            previous_marker = marker;
        }

        if self.frame.is_none() {
            return Err(Error::Format("end of image encountered before frame".to_owned()));
        }

        let frame = self.frame.as_ref().unwrap();

        // If we're decoding a progressive jpeg and a component is unfinished, render what we've got
        if frame.coding_process == CodingProcess::DctProgressive && self.coefficients.len() == frame.components.len() {
            for (i, component) in frame.components.iter().enumerate() {
                // Only dealing with unfinished components
                if self.coefficients_finished[i] == !0 {
                    continue;
                }

                let quantization_table = match self.quantization_tables[component.quantization_table_index].clone() {
                    Some(quantization_table) => quantization_table,
                    None => continue,
                };

                // Get the worker prepared
                if worker.is_none() {
                    worker = Some(PlatformWorker::new()?);
                }
                let worker = worker.as_mut().unwrap();
                let row_data = RowData {
                    index: i,
                    component: component.clone(),
                    quantization_table,
                };
                worker.start(row_data)?;

                // Send the rows over to the worker and collect the result
                let coefficients_per_mcu_row = usize::from(component.block_size.width) * usize::from(component.vertical_sampling_factor) * 64;
                for mcu_y in 0..frame.mcu_size.height {
                    let row_coefficients = {
                        let offset = usize::from(mcu_y) * coefficients_per_mcu_row;
                        self.coefficients[i][offset .. offset + coefficients_per_mcu_row].to_vec()
                    };

                    worker.append_row((i, row_coefficients))?;
                }
                planes[i] = worker.get_result(i)?;
            }
        }

        compute_image(&frame.components, planes, frame.output_size, self.is_jfif, self.color_transform)
    }

    fn read_marker(&mut self) -> Result<Marker> {
        loop {
            // This should be an error as the JPEG spec doesn't allow extraneous data between marker segments.
            // libjpeg allows this though and there are images in the wild utilising it, so we are
            // forced to support this behavior.
            // Sony Ericsson P990i is an example of a device which produce this sort of JPEGs.
            while read_u8(&mut self.reader)? != 0xFF {}

            // Section B.1.1.2
            // All markers are assigned two-byte codes: an X’FF’ byte followed by a
            // byte which is not equal to 0 or X’FF’ (see Table B.1). Any marker may
            // optionally be preceded by any number of fill bytes, which are bytes
            // assigned code X’FF’.
            let mut byte = read_u8(&mut self.reader)?;

            // Section B.1.1.2
            // "Any marker may optionally be preceded by any number of fill bytes, which are bytes assigned code X’FF’."
            while byte == 0xFF {
                byte = read_u8(&mut self.reader)?;
            }

            if byte != 0x00 && byte != 0xFF {
                return Ok(Marker::from_u8(byte).unwrap());
            }
        }
    }

    fn decode_scan(&mut self,
                   frame: &FrameInfo,
                   scan: &ScanInfo,
                   worker: &mut PlatformWorker,
                   finished: &[bool; MAX_COMPONENTS])
                   -> Result<(Option<Marker>, Option<Vec<Vec<u8>>>)> {
        assert!(scan.component_indices.len() <= MAX_COMPONENTS);

        let components: Vec<Component> = scan.component_indices.iter()
                                                               .map(|&i| frame.components[i].clone())
                                                               .collect();

        // Verify that all required quantization tables has been set.
        if components.iter().any(|component| self.quantization_tables[component.quantization_table_index].is_none()) {
            return Err(Error::Format("use of unset quantization table".to_owned()));
        }

        if self.is_mjpeg {
            fill_default_mjpeg_tables(scan, &mut self.dc_huffman_tables, &mut self.ac_huffman_tables);
        }

        // Verify that all required huffman tables has been set.
        if scan.spectral_selection.start == 0 &&
                scan.dc_table_indices.iter().any(|&i| self.dc_huffman_tables[i].is_none()) {
            return Err(Error::Format("scan makes use of unset dc huffman table".to_owned()));
        }
        if scan.spectral_selection.end > 1 &&
                scan.ac_table_indices.iter().any(|&i| self.ac_huffman_tables[i].is_none()) {
            return Err(Error::Format("scan makes use of unset ac huffman table".to_owned()));
        }

        // Prepare the worker thread for the work to come.
        for (i, component) in components.iter().enumerate() {
            if finished[i] {
                let row_data = RowData {
                    index: i,
                    component: component.clone(),
                    quantization_table: self.quantization_tables[component.quantization_table_index].clone().unwrap(),
                };

                worker.start(row_data)?;
            }
        }

        let is_progressive = frame.coding_process == CodingProcess::DctProgressive;
        let is_interleaved = components.len() > 1;
        let mut dummy_block = [0i16; 64];
        let mut huffman = HuffmanDecoder::new();
        let mut dc_predictors = [0i16; MAX_COMPONENTS];
        let mut mcus_left_until_restart = self.restart_interval;
        let mut expected_rst_num = 0;
        let mut eob_run = 0;
        let mut mcu_row_coefficients = Vec::with_capacity(components.len());

        if !is_progressive {
            for (_, component) in components.iter().enumerate().filter(|&(i, _)| finished[i]) {
                let coefficients_per_mcu_row = component.block_size.width as usize * component.vertical_sampling_factor as usize * 64;
                mcu_row_coefficients.push(vec![0i16; coefficients_per_mcu_row]);
            }
        }

        // 4.8.2
        // When reading from the stream, if the data is non-interleaved then an MCU consists of
        // exactly one block (effectively a 1x1 sample).
        let (mcu_horizontal_samples, mcu_vertical_samples) = if is_interleaved {
            let horizontal = components.iter().map(|component| component.horizontal_sampling_factor as u16).collect::<Vec<_>>();
            let vertical = components.iter().map(|component| component.vertical_sampling_factor as u16).collect::<Vec<_>>();
            (horizontal, vertical)
        } else {
            (vec![1], vec![1])
        };

        // This also affects how many MCU values we read from stream. If it's a non-interleaved stream,
        // the MCUs will be exactly the block count.
        let (max_mcu_x, max_mcu_y) = if is_interleaved {
            (frame.mcu_size.width, frame.mcu_size.height)
        } else {
            (components[0].block_size.width, components[0].block_size.height)
        };

        for mcu_y in 0..max_mcu_y {
            if mcu_y * 8 >= frame.image_size.height {
                break;
            }

            for mcu_x in 0..max_mcu_x {
                if mcu_x * 8 >= frame.image_size.width {
                    break;
                }

                if self.restart_interval > 0 {
                    if mcus_left_until_restart == 0 {
                        match huffman.take_marker(&mut self.reader)? {
                            Some(Marker::RST(n)) => {
                                if n != expected_rst_num {
                                    return Err(Error::Format(format!("found RST{} where RST{} was expected", n, expected_rst_num)));
                                }

                                huffman.reset();
                                // Section F.2.1.3.1
                                dc_predictors = [0i16; MAX_COMPONENTS];
                                // Section G.1.2.2
                                eob_run = 0;

                                expected_rst_num = (expected_rst_num + 1) % 8;
                                mcus_left_until_restart = self.restart_interval;
                            },
                            Some(marker) => return Err(Error::Format(format!("found marker {:?} inside scan where RST{} was expected", marker, expected_rst_num))),
                            None => return Err(Error::Format(format!("no marker found where RST{} was expected", expected_rst_num))),
                        }
                    }

                    mcus_left_until_restart -= 1;
                }

                for (i, component) in components.iter().enumerate() {
                    for v_pos in 0..mcu_vertical_samples[i] {
                        for h_pos in 0..mcu_horizontal_samples[i] {
                            let coefficients = if is_progressive {
                                let block_y = (mcu_y * mcu_vertical_samples[i] + v_pos) as usize;
                                let block_x = (mcu_x * mcu_horizontal_samples[i] + h_pos) as usize;
                                let block_offset = (block_y * component.block_size.width as usize + block_x) * 64;
                                &mut self.coefficients[scan.component_indices[i]][block_offset..block_offset + 64]
                            } else if finished[i] {
                                // Because the worker thread operates in batches as if we were always interleaved, we
                                // need to distinguish between a single-shot buffer and one that's currently in process
                                // (for a non-interleaved) stream
                                let mcu_batch_current_row = if is_interleaved {
                                    0
                                } else {
                                    mcu_y % component.vertical_sampling_factor as u16
                                };

                                let block_y = (mcu_batch_current_row * mcu_vertical_samples[i] + v_pos) as usize;
                                let block_x = (mcu_x * mcu_horizontal_samples[i] + h_pos) as usize;
                                let block_offset = (block_y * component.block_size.width as usize + block_x) * 64;
                                &mut mcu_row_coefficients[i][block_offset..block_offset + 64]
                            } else {
                                &mut dummy_block[..]
                            };

                            if scan.successive_approximation_high == 0 {
                                decode_block(&mut self.reader,
                                            coefficients,
                                            &mut huffman,
                                            self.dc_huffman_tables[scan.dc_table_indices[i]].as_ref(),
                                            self.ac_huffman_tables[scan.ac_table_indices[i]].as_ref(),
                                            scan.spectral_selection.clone(),
                                            scan.successive_approximation_low,
                                            &mut eob_run,
                                            &mut dc_predictors[i])?;
                            }
                            else {
                                decode_block_successive_approximation(&mut self.reader,
                                                                    coefficients,
                                                                    &mut huffman,
                                                                    self.ac_huffman_tables[scan.ac_table_indices[i]].as_ref(),
                                                                    scan.spectral_selection.clone(),
                                                                    scan.successive_approximation_low,
                                                                    &mut eob_run)?;
                            }
                        }
                    }
                }
            }

            // Send the coefficients from this MCU row to the worker thread for dequantization and idct.
            for (i, component) in components.iter().enumerate() {
                if finished[i] {
                    // In the event of non-interleaved streams, if we're still building the buffer out,
                    // keep going; don't send it yet. We also need to ensure we don't skip over the last
                    // row(s) of the image.
                    if !is_interleaved && (mcu_y + 1) * 8 < frame.image_size.height {
                        if (mcu_y + 1) % component.vertical_sampling_factor as u16 > 0 {
                            continue;
                        }
                    }

                    let coefficients_per_mcu_row = component.block_size.width as usize * component.vertical_sampling_factor as usize * 64;

                    let row_coefficients = if is_progressive {
                        // Because non-interleaved streams will have multiple MCU rows concatenated together,
                        // the row for calculating the offset is different.
                        let worker_mcu_y = if is_interleaved {
                            mcu_y
                        } else {
                            // Explicitly doing floor-division here
                            mcu_y / component.vertical_sampling_factor as u16
                        };

                        let offset = worker_mcu_y as usize * coefficients_per_mcu_row;
                        self.coefficients[scan.component_indices[i]][offset .. offset + coefficients_per_mcu_row].to_vec()
                    } else {
                        mem::replace(&mut mcu_row_coefficients[i], vec![0i16; coefficients_per_mcu_row])
                    };

                    worker.append_row((i, row_coefficients))?;
                }
            }
        }

        let mut marker = huffman.take_marker(&mut self.reader)?;
        while let Some(Marker::RST(_)) = marker {
            marker = self.read_marker().ok();
        }

        if finished.iter().any(|&c| c) {
            // Retrieve all the data from the worker thread.
            let mut data = vec![Vec::new(); frame.components.len()];

            for (i, &component_index) in scan.component_indices.iter().enumerate() {
                if finished[i] {
                    data[component_index] = worker.get_result(i)?;
                }
            }

            Ok((marker, Some(data)))
        }
        else {
            Ok((marker, None))
        }
    }
}

fn decode_block<R: Read>(reader: &mut R,
                         coefficients: &mut [i16],
                         huffman: &mut HuffmanDecoder,
                         dc_table: Option<&HuffmanTable>,
                         ac_table: Option<&HuffmanTable>,
                         spectral_selection: Range<u8>,
                         successive_approximation_low: u8,
                         eob_run: &mut u16,
                         dc_predictor: &mut i16) -> Result<()> {
    debug_assert_eq!(coefficients.len(), 64);

    if spectral_selection.start == 0 {
        // Section F.2.2.1
        // Figure F.12
        let value = huffman.decode(reader, dc_table.unwrap())?;
        let diff = match value {
            0 => 0,
            1..=11 => huffman.receive_extend(reader, value)?,
            _ => {
                // Section F.1.2.1.1
                // Table F.1
                return Err(Error::Format("invalid DC difference magnitude category".to_owned()));
            },
        };

        // Malicious JPEG files can cause this add to overflow, therefore we use wrapping_add.
        // One example of such a file is tests/crashtest/images/dc-predictor-overflow.jpg
        *dc_predictor = dc_predictor.wrapping_add(diff);
        coefficients[0] = *dc_predictor << successive_approximation_low;
    }

    let mut index = cmp::max(spectral_selection.start, 1);

    if index < spectral_selection.end && *eob_run > 0 {
        *eob_run -= 1;
        return Ok(());
    }

    // Section F.1.2.2.1
    while index < spectral_selection.end {
        if let Some((value, run)) = huffman.decode_fast_ac(reader, ac_table.unwrap())? {
            index += run;

            if index >= spectral_selection.end {
                break;
            }

            coefficients[UNZIGZAG[index as usize] as usize] = value << successive_approximation_low;
            index += 1;
        }
        else {
            let byte = huffman.decode(reader, ac_table.unwrap())?;
            let r = byte >> 4;
            let s = byte & 0x0f;

            if s == 0 {
                match r {
                    15 => index += 16, // Run length of 16 zero coefficients.
                    _  => {
                        *eob_run = (1 << r) - 1;

                        if r > 0 {
                            *eob_run += huffman.get_bits(reader, r)?;
                        }

                        break;
                    },
                }
            }
            else {
                index += r;

                if index >= spectral_selection.end {
                    break;
                }

                coefficients[UNZIGZAG[index as usize] as usize] = huffman.receive_extend(reader, s)? << successive_approximation_low;
                index += 1;
            }
        }
    }

    Ok(())
}

fn decode_block_successive_approximation<R: Read>(reader: &mut R,
                                                  coefficients: &mut [i16],
                                                  huffman: &mut HuffmanDecoder,
                                                  ac_table: Option<&HuffmanTable>,
                                                  spectral_selection: Range<u8>,
                                                  successive_approximation_low: u8,
                                                  eob_run: &mut u16) -> Result<()> {
    debug_assert_eq!(coefficients.len(), 64);

    let bit = 1 << successive_approximation_low;

    if spectral_selection.start == 0 {
        // Section G.1.2.1

        if huffman.get_bits(reader, 1)? == 1 {
            coefficients[0] |= bit;
        }
    }
    else {
        // Section G.1.2.3

        if *eob_run > 0 {
            *eob_run -= 1;
            refine_non_zeroes(reader, coefficients, huffman, spectral_selection, 64, bit)?;
            return Ok(());
        }

        let mut index = spectral_selection.start;

        while index < spectral_selection.end {
            let byte = huffman.decode(reader, ac_table.unwrap())?;
            let r = byte >> 4;
            let s = byte & 0x0f;

            let mut zero_run_length = r;
            let mut value = 0;

            match s {
                0 => {
                    match r {
                        15 => {
                            // Run length of 16 zero coefficients.
                            // We don't need to do anything special here, zero_run_length is 15
                            // and then value (which is zero) gets written, resulting in 16
                            // zero coefficients.
                        },
                        _ => {
                            *eob_run = (1 << r) - 1;

                            if r > 0 {
                                *eob_run += huffman.get_bits(reader, r)?;
                            }

                            // Force end of block.
                            zero_run_length = 64;
                        },
                    }
                },
                1 => {
                    if huffman.get_bits(reader, 1)? == 1 {
                        value = bit;
                    }
                    else {
                        value = -bit;
                    }
                },
                _ => return Err(Error::Format("unexpected huffman code".to_owned())),
            }

            let range = Range {
                start: index,
                end: spectral_selection.end,
            };
            index = refine_non_zeroes(reader, coefficients, huffman, range, zero_run_length, bit)?;

            if value != 0 {
                coefficients[UNZIGZAG[index as usize] as usize] = value;
            }

            index += 1;
        }
    }

    Ok(())
}

fn refine_non_zeroes<R: Read>(reader: &mut R,
                              coefficients: &mut [i16],
                              huffman: &mut HuffmanDecoder,
                              range: Range<u8>,
                              zrl: u8,
                              bit: i16) -> Result<u8> {
    debug_assert_eq!(coefficients.len(), 64);

    let last = range.end - 1;
    let mut zero_run_length = zrl;

    for i in range {
        let index = UNZIGZAG[i as usize] as usize;

        if coefficients[index] == 0 {
            if zero_run_length == 0 {
                return Ok(i);
            }

            zero_run_length -= 1;
        }
        else if huffman.get_bits(reader, 1)? == 1 && coefficients[index] & bit == 0 {
            if coefficients[index] > 0 {
                coefficients[index] += bit;
            }
            else {
                coefficients[index] -= bit;
            }
        }
    }

    Ok(last)
}

fn compute_image(components: &[Component],
                 mut data: Vec<Vec<u8>>,
                 output_size: Dimensions,
                 is_jfif: bool,
                 color_transform: Option<AdobeColorTransform>) -> Result<Vec<u8>> {
    if data.is_empty() || data.iter().any(Vec::is_empty) {
        return Err(Error::Format("not all components have data".to_owned()));
    }

    if components.len() == 1 {
        let component = &components[0];
        let mut decoded: Vec<u8> = data.remove(0);

        let width = component.size.width as usize;
        let height = component.size.height as usize;
        let size = width * height;
        let line_stride = component.block_size.width as usize * component.dct_scale;

        // if the image width is a multiple of the block size,
        // then we don't have to move bytes in the decoded data
        if usize::from(output_size.width) != line_stride {
            let mut buffer = vec![0u8; width];
            // The first line already starts at index 0, so we need to move only lines 1..height
            for y in 1..height {
                let destination_idx = y * width;
                let source_idx = y * line_stride;
                // We could use copy_within, but we need to support old rust versions
                buffer.copy_from_slice(&decoded[source_idx..][..width]);
                let destination = &mut decoded[destination_idx..][..width];
                destination.copy_from_slice(&buffer);
            }
        }
        decoded.resize(size, 0);
        Ok(decoded)
    }
    else {
        compute_image_parallel(components, data, output_size, is_jfif, color_transform)
    }
}

#[cfg(feature="rayon")]
fn compute_image_parallel(components: &[Component],
                          data: Vec<Vec<u8>>,
                          output_size: Dimensions,
                          is_jfif: bool,
                          color_transform: Option<AdobeColorTransform>) -> Result<Vec<u8>> {
    use rayon::prelude::*;

    let color_convert_func = choose_color_convert_func(components.len(), is_jfif, color_transform)?;
    let upsampler = Upsampler::new(components, output_size.width, output_size.height)?;
    let line_size = output_size.width as usize * components.len();
    let mut image = vec![0u8; line_size * output_size.height as usize];

    image.par_chunks_mut(line_size)
         .with_max_len(1)
         .enumerate()
         .for_each(|(row, line)| {
             upsampler.upsample_and_interleave_row(&data, row, output_size.width as usize, line);
             color_convert_func(line);
         });

    Ok(image)
 }

#[cfg(not(feature="rayon"))]
fn compute_image_parallel(components: &[Component],
                          data: Vec<Vec<u8>>,
                          output_size: Dimensions,
                          is_jfif: bool,
                          color_transform: Option<AdobeColorTransform>) -> Result<Vec<u8>> {
    let color_convert_func = choose_color_convert_func(components.len(), is_jfif, color_transform)?;
    let upsampler = Upsampler::new(components, output_size.width, output_size.height)?;
    let line_size = output_size.width as usize * components.len();
    let mut image = vec![0u8; line_size * output_size.height as usize];

    for (row, line) in image.chunks_mut(line_size)
         .enumerate() {
             upsampler.upsample_and_interleave_row(&data, row, output_size.width as usize, line);
             color_convert_func(line);
         }

    Ok(image)
}

fn choose_color_convert_func(component_count: usize,
                             _is_jfif: bool,
                             color_transform: Option<AdobeColorTransform>)
                             -> Result<fn(&mut [u8])> {
    match component_count {
        3 => {
            // http://www.sno.phy.queensu.ca/~phil/exiftool/TagNames/JPEG.html#Adobe
            // Unknown means the data is RGB, so we don't need to perform any color conversion on it.
            if color_transform == Some(AdobeColorTransform::Unknown) {
                Ok(color_convert_line_null)
            }
            else {
                Ok(color_convert_line_ycbcr)
            }
        },
        4 => {
            // http://www.sno.phy.queensu.ca/~phil/exiftool/TagNames/JPEG.html#Adobe
            match color_transform {
                Some(AdobeColorTransform::Unknown) => Ok(color_convert_line_cmyk),
                Some(_) => Ok(color_convert_line_ycck),
                None => Err(Error::Format("4 components without Adobe APP14 metadata to indicate color space".to_owned())),
            }
        },
        _ => panic!(),
    }
}

fn color_convert_line_null(_data: &mut [u8]) {
}

fn color_convert_line_ycbcr(data: &mut [u8]) {
    for chunk in data.chunks_exact_mut(3) {
        let (r, g, b) = ycbcr_to_rgb(chunk[0], chunk[1], chunk[2]);
        chunk[0] = r;
        chunk[1] = g;
        chunk[2] = b;
    }
}

fn color_convert_line_ycck(data: &mut [u8]) {
    for chunk in data.chunks_exact_mut(4) {
        let (r, g, b) = ycbcr_to_rgb(chunk[0], chunk[1], chunk[2]);
        let k = chunk[3];
        chunk[0] = r;
        chunk[1] = g;
        chunk[2] = b;
        chunk[3] = 255 - k;

    }
}

fn color_convert_line_cmyk(data: &mut [u8]) {
    for chunk in data.chunks_exact_mut(4) {
        chunk[0] = 255 - chunk[0];
        chunk[1] = 255 - chunk[1];
        chunk[2] = 255 - chunk[2];
        chunk[3] = 255 - chunk[3];
    }
}

// ITU-R BT.601
fn ycbcr_to_rgb(y: u8, cb: u8, cr: u8) -> (u8, u8, u8) {
    let y = y as f32;
    let cb = cb as f32 - 128.0;
    let cr = cr as f32 - 128.0;

    let r = y                + 1.40200 * cr;
    let g = y - 0.34414 * cb - 0.71414 * cr;
    let b = y + 1.77200 * cb;

    // TODO: Rust has defined float-to-int conversion as saturating,
    // which is exactly what we need here. However, as of this writing
    // it still hasn't reached the stable channel.
    // This can be simplified to `(r + 0.5) as u8` without any clamping
    // as soon as our MSRV reaches the version that has saturating casts.
    // The version without explicit clamping is also noticeably faster.
    (clamp_to_u8((r + 0.5) as i32) as u8,
     clamp_to_u8((g + 0.5) as i32) as u8,
     clamp_to_u8((b + 0.5) as i32) as u8)
}

fn clamp_to_u8(value: i32) -> i32 {
    let value = std::cmp::max(value, 0);
    std::cmp::min(value, 255)
}
