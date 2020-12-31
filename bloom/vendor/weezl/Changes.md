## Version 0.1.3

- Fixes an issue in compression that caused some data to be lost around clear
  codes. This could corrupt the data stream.

## Version 0.1.2

- Fixes incorrect compression after `Encoder::reset`.

## Version 0.1.1 

- The `IntoStream` types now reuse their internal buffers.
- Added the methods `set_buffer`, `set_buffer_size` to `IntoStream` for both
  the encoder and decoder, used to control the automatic allocation.
- Deprecated `IntoStream` in configurations without the `std` feature where the
  type can't even be constructed.

## Version 0.1.0 – Aleph

- Initial major release
- Support gif and tiff code size changes
- Rough performance numbers:
  On i5-4690, 8GiB DIMM DDR3 Synchronous 1600 MHz (0,6 ns)
  ~70MB/s encode, ~230MB/s decode
