use nom::{bytes::complete::take, IResult};

use super::{ColorMap, size::Size, Bpp, DataType, ImageOrigin, TgaError, TgaHeader};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Tga<'a> {
    header: TgaHeader,
    pixels: &'a [u8],
}

impl<'a> Tga<'a> {
    pub fn from_slice(data: &'a [u8]) -> Result<Self, TgaError> {
        let input = data;
        let (input, header) = TgaHeader::parse(input).map_err(|_| TgaError::Header)?;
        let (input, _image_id) = parse_image_id(input, &header).map_err(|_| TgaError::Header)?;
        let (input, color_map) = ColorMap::parse(input, &header)?;

        Ok(Self {
            header,
            pixels: data,
        })
    }
}

fn parse_image_id<'a>(input: &'a [u8], header: &TgaHeader) -> IResult<&'a [u8], &'a [u8]> {
    take(header.id_len)(input)
}
