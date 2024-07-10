use nom::{bytes::complete::take};

use super::{TgaError, TgaHeader};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ColorMap<'a> {
	data: &'a [u8]
}

impl<'a> ColorMap<'a> {
	pub fn parse(input: &'a [u8], header: &TgaHeader) -> Result<(&'a [u8], Option<Self>), TgaError> {
		if !header.has_color_map {
			return Ok((input, None));
		}
		let entry_bpp = header.color_map_depth.ok_or(TgaError::ColorMap)?;
		let length = usize::from(header.color_map_len) * usize::from(entry_bpp.bytes());

		let (input, color_map_data) = take(length)(input).map_err(|_: nom::Err<()>| TgaError::ColorMap)?;
		Ok((input, Some(Self {
			data: color_map_data
		})))
	}
}
