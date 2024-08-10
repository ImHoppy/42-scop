use super::{Bpp, DataType, ImageOrigin, Tga, TgaHeader};

#[test]
pub fn chessboard_4px_raw() {
	let data = include_bytes!("./tests/chessboard_4px.tga");

	let img = Tga::from_slice(data).unwrap();

	println!("{:#?}", img.header());
	println!("Raw image data len {:#?}", img.image_data().len());
	println!("Raw image data {:#?}", img.image_data());

	assert_eq!(
		img.header(),
		TgaHeader {
			id_len: 0,
			has_color_map: false,
			data_type: DataType::TrueColor,
			compressed: false,
			color_map_start: 0,
			color_map_len: 0,
			color_map_depth: None,
			x_origin: 0,
			y_origin: 4,
			width: 4,
			height: 4,
			pixel_depth: Bpp::Bits24,
			image_origin: ImageOrigin::TopLeft,
			alpha_channel_depth: 0,
		}
	);

	let pixels = img.image_data();

	assert_eq!(pixels.len(), 4 * 4);
	assert_eq!(
		pixels,
		&vec![
			0x00ffffffu32,
			0x00000000,
			0x00ffffff,
			0x00000000,
			0x00000000,
			0x00ff0000,
			0x00000000,
			0x0000ff00,
			0x00ffffff,
			0x00000000,
			0x000000ff,
			0x00000000,
			0x00000000,
			0x00ffffff,
			0x00000000,
			0x00ffffff,
		]
	);
}