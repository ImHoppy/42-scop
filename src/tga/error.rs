#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TgaError {
    /// An error occurred when parsing the TGA header.
	Header,
	/// Unknown ColorMap
	ColorMap,
    /// An unsupported image type value was encountered.
	UnknownImageType(u8),
	/// Not supported compressed
	CompressedNotImplemented,
	/// Error parsing image data
	ParseImageData,
}
