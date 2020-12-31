use byteorder::{BigEndian, ReadBytesExt};
use error::{Error, Result};
use huffman::{HuffmanTable, HuffmanTableClass};
use marker::Marker;
use marker::Marker::*;
use std::io::{self, Read};
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntropyCoding {
    Huffman,
    Arithmetic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodingProcess {
    DctSequential,
    DctProgressive,
    Lossless,
}

#[derive(Clone)]
pub struct FrameInfo {
    pub is_baseline: bool,
    pub is_differential: bool,
    pub coding_process: CodingProcess,
    pub entropy_coding: EntropyCoding,
    pub precision: u8,

    pub image_size: Dimensions,
    pub output_size: Dimensions,
    pub mcu_size: Dimensions,
    pub components: Vec<Component>,
}

#[derive(Debug)]
pub struct ScanInfo {
    pub component_indices: Vec<usize>,
    pub dc_table_indices: Vec<usize>,
    pub ac_table_indices: Vec<usize>,

    pub spectral_selection: Range<u8>,
    pub successive_approximation_high: u8,
    pub successive_approximation_low: u8,
}

#[derive(Clone, Debug)]
pub struct Component {
    pub identifier: u8,

    pub horizontal_sampling_factor: u8,
    pub vertical_sampling_factor: u8,

    pub quantization_table_index: usize,

    pub dct_scale: usize,

    pub size: Dimensions,
    pub block_size: Dimensions,
}

#[derive(Debug)]
pub enum AppData {
    Adobe(AdobeColorTransform),
    Jfif,
    Avi1,
}

// http://www.sno.phy.queensu.ca/~phil/exiftool/TagNames/JPEG.html#Adobe
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdobeColorTransform {
    // RGB or CMYK
    Unknown,
    YCbCr,
    // YCbCrK
    YCCK,
}

impl FrameInfo {
    pub(crate) fn update_idct_size(&mut self, idct_size: usize) {
        for component in &mut self.components {
            component.dct_scale = idct_size;
        }

        update_component_sizes(self.image_size, &mut self.components);

        self.output_size = Dimensions {
            width: (self.image_size.width as f32 * idct_size as f32 / 8.0).ceil() as u16,
            height: (self.image_size.height as f32 * idct_size as f32 / 8.0).ceil() as u16
        };
    }
}

fn read_length<R: Read>(reader: &mut R, marker: Marker) -> Result<usize> {
    assert!(marker.has_length());

    // length is including itself.
    let length = reader.read_u16::<BigEndian>()? as usize;

    if length < 2 {
        return Err(Error::Format(format!("encountered {:?} with invalid length {}", marker, length)));
    }

    Ok(length - 2)
}

fn skip_bytes<R: Read>(reader: &mut R, length: usize) -> Result<()> {
    let length = length as u64;
    let to_skip = &mut reader.by_ref().take(length);
    let copied = io::copy(to_skip, &mut io::sink())?;
    if copied < length {
        Err(Error::Io(io::ErrorKind::UnexpectedEof.into()))
    } else {
        Ok(())
    }
}

// Section B.2.2
pub fn parse_sof<R: Read>(reader: &mut R, marker: Marker) -> Result<FrameInfo> {
    let length = read_length(reader, marker)?;

    if length <= 6 {
        return Err(Error::Format("invalid length in SOF".to_owned()));
    }

    let is_baseline = marker == SOF(0);
    let is_differential = match marker {
        SOF(0 ..= 3) | SOF(9 ..= 11)  => false,
        SOF(5 ..= 7) | SOF(13 ..= 15) => true,
        _ => panic!(),
    };
    let coding_process = match marker {
        SOF(0) | SOF(1) | SOF(5) | SOF(9) | SOF(13) => CodingProcess::DctSequential,
        SOF(2) | SOF(6) | SOF(10) | SOF(14)         => CodingProcess::DctProgressive,
        SOF(3) | SOF(7) | SOF(11) | SOF(15)         => CodingProcess::Lossless,
        _ => panic!(),
    };
    let entropy_coding = match marker {
        SOF(0 ..= 3) | SOF(5 ..= 7)     => EntropyCoding::Huffman,
        SOF(9 ..= 11) | SOF(13 ..= 15)  => EntropyCoding::Arithmetic,
        _ => panic!(),
    };

    let precision = reader.read_u8()?;

    match precision {
        8 => {},
        12 => {
            if is_baseline {
                return Err(Error::Format("12 bit sample precision is not allowed in baseline".to_owned()));
            }
        },
        _ => {
            if coding_process != CodingProcess::Lossless {
                return Err(Error::Format(format!("invalid precision {} in frame header", precision)))
            }
        },
    }

    let height = reader.read_u16::<BigEndian>()?;
    let width = reader.read_u16::<BigEndian>()?;

    // height:
    // "Value 0 indicates that the number of lines shall be defined by the DNL marker and
    //     parameters at the end of the first scan (see B.2.5)."

    if width == 0 {
        return Err(Error::Format("zero width in frame header".to_owned()));
    }

    let component_count = reader.read_u8()?;

    if component_count == 0 {
        return Err(Error::Format("zero component count in frame header".to_owned()));
    }
    if coding_process == CodingProcess::DctProgressive && component_count > 4 {
        return Err(Error::Format("progressive frame with more than 4 components".to_owned()));
    }

    if length != 6 + 3 * component_count as usize {
        return Err(Error::Format("invalid length in SOF".to_owned()));
    }

    let mut components: Vec<Component> = Vec::with_capacity(component_count as usize);

    for _ in 0 .. component_count {
        let identifier = reader.read_u8()?;

        // Each component's identifier must be unique.
        if components.iter().any(|c| c.identifier == identifier) {
            return Err(Error::Format(format!("duplicate frame component identifier {}", identifier)));
        }

        let byte = reader.read_u8()?;
        let horizontal_sampling_factor = byte >> 4;
        let vertical_sampling_factor = byte & 0x0f;

        if horizontal_sampling_factor == 0 || horizontal_sampling_factor > 4 {
            return Err(Error::Format(format!("invalid horizontal sampling factor {}", horizontal_sampling_factor)));
        }
        if vertical_sampling_factor == 0 || vertical_sampling_factor > 4 {
            return Err(Error::Format(format!("invalid vertical sampling factor {}", vertical_sampling_factor)));
        }

        let quantization_table_index = reader.read_u8()?;

        if quantization_table_index > 3 || (coding_process == CodingProcess::Lossless && quantization_table_index != 0) {
            return Err(Error::Format(format!("invalid quantization table index {}", quantization_table_index)));
        }

        components.push(Component {
            identifier: identifier,
            horizontal_sampling_factor: horizontal_sampling_factor,
            vertical_sampling_factor: vertical_sampling_factor,
            quantization_table_index: quantization_table_index as usize,
            dct_scale: 8,
            size: Dimensions {width: 0, height: 0},
            block_size: Dimensions {width: 0, height: 0},
        });
    }

    let mcu_size = update_component_sizes(Dimensions { width, height }, &mut components);

    Ok(FrameInfo {
        is_baseline: is_baseline,
        is_differential: is_differential,
        coding_process: coding_process,
        entropy_coding: entropy_coding,
        precision: precision,
        image_size: Dimensions { width, height },
        output_size: Dimensions { width, height },
        mcu_size,
        components: components,
    })
}

/// Returns ceil(x/y), requires x>0
fn ceil_div(x: u32, y: u32) -> u16 {
    assert!(x>0 && y>0, "invalid dimensions");
    (1 + ((x - 1) / y)) as u16
}

fn update_component_sizes(size: Dimensions, components: &mut [Component]) -> Dimensions {
    let h_max = components.iter().map(|c| c.horizontal_sampling_factor).max().unwrap() as u32;
    let v_max = components.iter().map(|c| c.vertical_sampling_factor).max().unwrap() as u32;

    let mcu_size = Dimensions {
        width: ceil_div(size.width as u32, h_max * 8),
        height: ceil_div(size.height as u32, v_max * 8),
    };

    for component in components {
        component.size.width = ceil_div(size.width as u32 * component.horizontal_sampling_factor as u32 * component.dct_scale as u32, h_max * 8);
        component.size.height = ceil_div(size.height as u32 * component.vertical_sampling_factor as u32 * component.dct_scale as u32, v_max * 8);

        component.block_size.width = mcu_size.width * component.horizontal_sampling_factor as u16;
        component.block_size.height = mcu_size.height * component.vertical_sampling_factor as u16;
    }

    mcu_size
}

#[test]
fn test_update_component_sizes() {
    let mut components = [Component {
        identifier: 1,
        horizontal_sampling_factor: 2,
        vertical_sampling_factor: 2,
        quantization_table_index: 0,
        dct_scale: 8,
        size: Dimensions { width: 0, height: 0 },
        block_size: Dimensions { width: 0, height: 0 },
    }];
    let mcu = update_component_sizes(
        Dimensions { width: 800, height: 280 },
        &mut components);
    assert_eq!(mcu, Dimensions { width: 50, height: 18 });
    assert_eq!(components[0].block_size, Dimensions { width: 100, height: 36 });
    assert_eq!(components[0].size, Dimensions { width: 800, height: 280 });
}

// Section B.2.3
pub fn parse_sos<R: Read>(reader: &mut R, frame: &FrameInfo) -> Result<ScanInfo> {
    let length = read_length(reader, SOS)?;
    if 0 == length {
        return Err(Error::Format("zero length in SOS".to_owned()));
    }

    let component_count = reader.read_u8()?;

    if component_count == 0 || component_count > 4 {
        return Err(Error::Format(format!("invalid component count {} in scan header", component_count)));
    }

    if length != 4 + 2 * component_count as usize {
        return Err(Error::Format("invalid length in SOS".to_owned()));
    }

    let mut component_indices = Vec::with_capacity(component_count as usize);
    let mut dc_table_indices = Vec::with_capacity(component_count as usize);
    let mut ac_table_indices = Vec::with_capacity(component_count as usize);

    for _ in 0 .. component_count {
        let identifier = reader.read_u8()?;

        let component_index = match frame.components.iter().position(|c| c.identifier == identifier) {
            Some(value) => value,
            None => return Err(Error::Format(format!("scan component identifier {} does not match any of the component identifiers defined in the frame", identifier))),
        };

        // Each of the scan's components must be unique.
        if component_indices.contains(&component_index) {
            return Err(Error::Format(format!("duplicate scan component identifier {}", identifier)));
        }

        // "... the ordering in the scan header shall follow the ordering in the frame header."
        if component_index < *component_indices.iter().max().unwrap_or(&0) {
            return Err(Error::Format("the scan component order does not follow the order in the frame header".to_owned()));
        }

        let byte = reader.read_u8()?;
        let dc_table_index = byte >> 4;
        let ac_table_index = byte & 0x0f;

        if dc_table_index > 3 || (frame.is_baseline && dc_table_index > 1) {
            return Err(Error::Format(format!("invalid dc table index {}", dc_table_index)));
        }
        if ac_table_index > 3 || (frame.is_baseline && ac_table_index > 1) {
            return Err(Error::Format(format!("invalid ac table index {}", ac_table_index)));
        }

        component_indices.push(component_index);
        dc_table_indices.push(dc_table_index as usize);
        ac_table_indices.push(ac_table_index as usize);
    }

    let blocks_per_mcu = component_indices.iter().map(|&i| {
        frame.components[i].horizontal_sampling_factor as u32 * frame.components[i].vertical_sampling_factor as u32
    }).fold(0, ::std::ops::Add::add);

    if component_count > 1 && blocks_per_mcu > 10 {
        return Err(Error::Format("scan with more than one component and more than 10 blocks per MCU".to_owned()));
    }

    let spectral_selection_start = reader.read_u8()?;
    let spectral_selection_end = reader.read_u8()?;

    let byte = reader.read_u8()?;
    let successive_approximation_high = byte >> 4;
    let successive_approximation_low = byte & 0x0f;

    if frame.coding_process == CodingProcess::DctProgressive {
        if spectral_selection_end > 63 || spectral_selection_start > spectral_selection_end ||
                (spectral_selection_start == 0 && spectral_selection_end != 0) {
            return Err(Error::Format(format!("invalid spectral selection parameters: ss={}, se={}", spectral_selection_start, spectral_selection_end)));
        }
        if spectral_selection_start != 0 && component_count != 1 {
            return Err(Error::Format("spectral selection scan with AC coefficients can't have more than one component".to_owned()));
        }

        if successive_approximation_high > 13 || successive_approximation_low > 13 {
            return Err(Error::Format(format!("invalid successive approximation parameters: ah={}, al={}", successive_approximation_high, successive_approximation_low)));
        }

        // Section G.1.1.1.2
        // "Each scan which follows the first scan for a given band progressively improves
        //     the precision of the coefficients by one bit, until full precision is reached."
        if successive_approximation_high != 0 && successive_approximation_high != successive_approximation_low + 1 {
            return Err(Error::Format("successive approximation scan with more than one bit of improvement".to_owned()));
        }
    }
    else {
        if spectral_selection_start != 0 || spectral_selection_end != 63 {
            return Err(Error::Format("spectral selection is not allowed in non-progressive scan".to_owned()));
        }
        if successive_approximation_high != 0 || successive_approximation_low != 0 {
            return Err(Error::Format("successive approximation is not allowed in non-progressive scan".to_owned()));
        }
    }

    Ok(ScanInfo {
        component_indices: component_indices,
        dc_table_indices: dc_table_indices,
        ac_table_indices: ac_table_indices,
        spectral_selection: Range {
            start: spectral_selection_start,
            end: spectral_selection_end + 1,
        },
        successive_approximation_high: successive_approximation_high,
        successive_approximation_low: successive_approximation_low,
    })
}

// Section B.2.4.1
pub fn parse_dqt<R: Read>(reader: &mut R) -> Result<[Option<[u16; 64]>; 4]> {
    let mut length = read_length(reader, DQT)?;
    let mut tables = [None; 4];

    // Each DQT segment may contain multiple quantization tables.
    while length > 0 {
        let byte = reader.read_u8()?;
        let precision = (byte >> 4) as usize;
        let index = (byte & 0x0f) as usize;

        // The combination of 8-bit sample precision and 16-bit quantization tables is explicitly
        // disallowed by the JPEG spec:
        //     "An 8-bit DCT-based process shall not use a 16-bit precision quantization table."
        //     "Pq: Quantization table element precision – Specifies the precision of the Qk
        //      values. Value 0 indicates 8-bit Qk values; value 1 indicates 16-bit Qk values. Pq
        //      shall be zero for 8 bit sample precision P (see B.2.2)."
        // libjpeg allows this behavior though, and there are images in the wild using it. So to
        // match libjpeg's behavior we are deviating from the JPEG spec here.
        if precision > 1 {
            return Err(Error::Format(format!("invalid precision {} in DQT", precision)));
        }
        if index > 3 {
            return Err(Error::Format(format!("invalid destination identifier {} in DQT", index)));
        }
        if length < 65 + 64 * precision {
            return Err(Error::Format("invalid length in DQT".to_owned()));
        }

        let mut table = [0u16; 64];

        for i in 0 .. 64 {
            table[i] = match precision {
                0 => reader.read_u8()? as u16,
                1 => reader.read_u16::<BigEndian>()?,
                _ => unreachable!(),
            };
        }

        if table.iter().any(|&val| val == 0) {
            return Err(Error::Format("quantization table contains element with a zero value".to_owned()));
        }

        tables[index] = Some(table);
        length -= 65 + 64 * precision;
    }

    Ok(tables)
}

// Section B.2.4.2
pub fn parse_dht<R: Read>(reader: &mut R, is_baseline: Option<bool>) -> Result<(Vec<Option<HuffmanTable>>, Vec<Option<HuffmanTable>>)> {
    let mut length = read_length(reader, DHT)?;
    let mut dc_tables = vec![None, None, None, None];
    let mut ac_tables = vec![None, None, None, None];

    // Each DHT segment may contain multiple huffman tables.
    while length > 17 {
        let byte = reader.read_u8()?;
        let class = byte >> 4;
        let index = (byte & 0x0f) as usize;

        if class != 0 && class != 1 {
            return Err(Error::Format(format!("invalid class {} in DHT", class)));
        }
        if is_baseline == Some(true) && index > 1 {
            return Err(Error::Format("a maximum of two huffman tables per class are allowed in baseline".to_owned()));
        }
        if index > 3 {
            return Err(Error::Format(format!("invalid destination identifier {} in DHT", index)));
        }

        let mut counts = [0u8; 16];
        reader.read_exact(&mut counts)?;

        let size = counts.iter().map(|&val| val as usize).fold(0, ::std::ops::Add::add);

        if size == 0 {
            return Err(Error::Format("encountered table with zero length in DHT".to_owned()));
        }
        else if size > 256 {
            return Err(Error::Format("encountered table with excessive length in DHT".to_owned()));
        }
        else if size > length - 17 {
            return Err(Error::Format("invalid length in DHT".to_owned()));
        }

        let mut values = vec![0u8; size];
        reader.read_exact(&mut values)?;

        match class {
            0 => dc_tables[index] = Some(HuffmanTable::new(&counts, &values, HuffmanTableClass::DC)?),
            1 => ac_tables[index] = Some(HuffmanTable::new(&counts, &values, HuffmanTableClass::AC)?),
            _ => unreachable!(),
        }

        length -= 17 + size;
    }

    if length != 0 {
        return Err(Error::Format("invalid length in DHT".to_owned()));
    }

    Ok((dc_tables, ac_tables))
}

// Section B.2.4.4
pub fn parse_dri<R: Read>(reader: &mut R) -> Result<u16> {
    let length = read_length(reader, DRI)?;

    if length != 2 {
        return Err(Error::Format("DRI with invalid length".to_owned()));
    }

    Ok(reader.read_u16::<BigEndian>()?)
}

// Section B.2.4.5
pub fn parse_com<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let length = read_length(reader, COM)?;
    let mut buffer = vec![0u8; length];

    reader.read_exact(&mut buffer)?;

    Ok(buffer)
}

// Section B.2.4.6
pub fn parse_app<R: Read>(reader: &mut R, marker: Marker) -> Result<Option<AppData>> {
    let length = read_length(reader, marker)?;
    let mut bytes_read = 0;
    let mut result = None;

    match marker {
        APP(0) => {
            if length >= 5 {
                let mut buffer = [0u8; 5];
                reader.read_exact(&mut buffer)?;
                bytes_read = buffer.len();

                // http://www.w3.org/Graphics/JPEG/jfif3.pdf
                if &buffer[0 .. 5] == &[b'J', b'F', b'I', b'F', b'\0'] {
                    result = Some(AppData::Jfif);
                // https://sno.phy.queensu.ca/~phil/exiftool/TagNames/JPEG.html#AVI1
                } else if &buffer[0 .. 5] == &[b'A', b'V', b'I', b'1', b'\0'] {
                    result = Some(AppData::Avi1);
                }
            }
        },
        APP(14) => {
            if length >= 12 {
                let mut buffer = [0u8; 12];
                reader.read_exact(&mut buffer)?;
                bytes_read = buffer.len();

                // http://www.sno.phy.queensu.ca/~phil/exiftool/TagNames/JPEG.html#Adobe
                if &buffer[0 .. 6] == &[b'A', b'd', b'o', b'b', b'e', b'\0'] {
                    let color_transform = match buffer[11] {
                        0 => AdobeColorTransform::Unknown,
                        1 => AdobeColorTransform::YCbCr,
                        2 => AdobeColorTransform::YCCK,
                        _ => return Err(Error::Format("invalid color transform in adobe app segment".to_owned())),
                    };

                    result = Some(AppData::Adobe(color_transform));
                }
            }
        },
        _ => {},
    }

    skip_bytes(reader, length - bytes_read)?;
    Ok(result)
}
