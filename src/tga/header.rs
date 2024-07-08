
/// Bits per pixel.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Bpp {
	Bits8,
	Bits16,
	Bits24,
	Bits32,
}

impl Bpp {
	fn new(value: u8) -> Option<Self> {
		Some(match value {
			8 => Self::Bits8,
			16 => Self::Bits16,
			24 => Self::Bits24,
			32 => Self::Bits32,
			_ => return None,
		})
	}
}

/// Image data type.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DataType {
    /// No image data.
    NoData,
    /// Color mapped.
    ColorMapped,
    /// True color.
    TrueColor,
    /// Black and white or grayscale.
    BlackAndWhite,
}

/// Image origin
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ImageOrigin {
	/// Bottom left
	BottomLeft,
	/// Bottom right
	BottomRight,
	/// Top left
	TopLeft,
	/// Top right
	TopRight,
}

impl ImageOrigin {
	fn new(value: u8) -> Self {
		match (value) {
			0 => Self::BottomLeft,
			1 => Self::BottomRight,
			2 => Self::TopLeft,
			_ => Self::TopRight,
		}
	}
}

/// TGA header.
///
/// See <https://www.fileformat.info/format/tga/egff.htm> for a detailed description of the fields.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct TgaHeader {
	/// Size of Image ID field
	pub id_len: u8,
	/// Has Color map type
	pub has_color_map: bool,
	/// Data Type
	pub data_type: DataType,
	/// Compressed, not implemented
	pub compressed: bool,
	/// Color Map start
	pub color_map_start: u16,
	/// Color Map length
	pub color_map_len: u16,
	/// Depth of color map entries
	pub color_map_depth: Option<Bpp>,
	/// X Origin
	pub x_origin: u16,
	/// Y Origin
	pub y_origin: u16,
    /// Image width in pixels
    pub width: u16,
    /// Image height in pixels
    pub height: u16,
    /// Pixel bit depth
    pub pixel_depth: Bpp,
    /// Image origin
    pub image_origin: ImageOrigin,
    /// Alpha channel depth
    pub alpha_channel_depth: u8,
}

impl TgaHeader {
	pub fn parse(input: &[u8]) -> Self {
		TgaHeader {

		}
	}
}
