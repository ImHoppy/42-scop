use super::{tga::Tga, Point};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pixels<'a> {
    tga: &'a Tga<'a>,
}

impl Iterator for Pixels<'_> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.next_position()?;

        let color = match &mut self.colors {
            DynamicRawColors::Bpp8Uncompressed(colors) => u32::from(colors.next()?.into_inner()),
            DynamicRawColors::Bpp8Rle(colors) => u32::from(colors.next()?.into_inner()),
            DynamicRawColors::Bpp16Uncompressed(colors) => u32::from(colors.next()?.into_inner()),
            DynamicRawColors::Bpp16Rle(colors) => u32::from(colors.next()?.into_inner()),
            DynamicRawColors::Bpp24Uncompressed(colors) => colors.next()?.into_inner(),
            DynamicRawColors::Bpp24Rle(colors) => colors.next()?.into_inner(),
            DynamicRawColors::Bpp32Uncompressed(colors) => colors.next()?.into_inner(),
            DynamicRawColors::Bpp32Rle(colors) => colors.next()?.into_inner(),
        };

        Some(Pixel::new(position, color))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pixel {
    /// The position relative to the top left corner of the image.
    pub position: Point,

    /// The raw pixel color.
    pub color: u32,
}

impl Pixel {
    pub const fn new(position: Point, color: u32) -> Self {
        Self { position, color }
    }
}
