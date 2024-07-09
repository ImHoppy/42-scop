use nom::{
    combinator::{map, map_opt, map_res},
    number::complete::{le_u16, le_u8},
    IResult,
};

use super::TgaError;

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

    /// Returns the number of bits.
    pub fn bits(&self) -> u8 {
        match self {
            Self::Bits8 => 8,
            Self::Bits16 => 16,
            Self::Bits24 => 24,
            Self::Bits32 => 32,
        }
    }

    /// Returns the number of bytes
    pub fn bytes(&self) -> u8 {
        match self {
            Self::Bits8 => 1,
            Self::Bits16 => 2,
            Self::Bits24 => 3,
            Self::Bits32 => 4,
        }
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

impl DataType {
    fn new(value: u8) -> Self {
        match value & 0x3 {
            1 => Self::ColorMapped,
            2 => Self::TrueColor,
            3 => Self::BlackAndWhite,
            _ => Self::NoData,
        }
    }
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
        match value & 0x3 {
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

fn parse_image_type(value: u8) -> Result<(DataType, bool), TgaError> {
    if value & !0b1011 != 0 {
        return Err(TgaError::UnknownImageType(value));
    }
    let data_type = DataType::new(value % 0x3);
    let compressed = value & 0x8 == 1;
    if compressed {
        return Err(TgaError::CompressedNotImplemented);
    }
    Ok((data_type, compressed))
}

fn parse_color_map(input: &[u8]) -> IResult<&[u8], bool> {
    map_res(le_u8, |b| match b {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(TgaError::ColorMap),
    })(input)
}

impl TgaHeader {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, id_len) = le_u8(input)?;
        let (input, has_color_map) = parse_color_map(input)?;
        let (input, (data_type, compressed)) = map_res(le_u8, parse_image_type)(input)?;
        let (input, color_map_start) = le_u16(input)?;
        let (input, color_map_len) = le_u16(input)?;
        let (input, color_map_depth) = map(le_u8, Bpp::new)(input)?;
        let (input, x_origin) = le_u16(input)?;
        let (input, y_origin) = le_u16(input)?;
        let (input, width) = le_u16(input)?;
        let (input, height) = le_u16(input)?;
        let (input, pixel_depth) = map_opt(le_u8, Bpp::new)(input)?;

        let (input, image_descriptor) = le_u8(input)?;
        let image_origin = ImageOrigin::new(image_descriptor >> 4);
        let alpha_channel_depth = image_descriptor & 0xF;

        Ok((
            input,
            TgaHeader {
                id_len,
                has_color_map,
                data_type,
                compressed,
                color_map_start,
                color_map_len,
                color_map_depth,
                x_origin,
                y_origin,
                width,
                height,
                pixel_depth,
                image_origin,
                alpha_channel_depth,
            },
        ))
    }
}
