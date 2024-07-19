use nom::{bytes::complete::take, IResult};

use super::{footer::TgaFooter, Bpp, ColorMap, DataType, ImageOrigin, TgaError, TgaHeader};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Tga<'a> {
    header: TgaHeader,
    pixels: &'a [u8],
    width: u16,
    height: u16,
}

impl<'a> Tga<'a> {
    pub fn from_slice(data: &'a [u8]) -> Result<Self, TgaError> {
        let input = data;
        let (input, header) = TgaHeader::parse(input).map_err(|_| TgaError::Header)?;
        let (input, _image_id) = parse_image_id(input, &header).map_err(|_| TgaError::Header)?;
        let (input, color_map) = ColorMap::parse(input, &header)?;

        let footer_length = TgaFooter::parse(input)
            .map(|footer| footer.length(data))
            .unwrap_or(0);

        let pixel_data = &[0..input.len().saturating_sub(footer_length)];

        Ok(Self {
            header,
            pixels: data,
            width: header.width,
            height: header.height,
        })
    }

    pub fn image_data(&self) -> &'a [u8] {
        self.pixels
    }
}

fn parse_image_id<'a>(input: &'a [u8], header: &TgaHeader) -> IResult<&'a [u8], &'a [u8]> {
    take(header.id_len)(input)
}
