pub use color_map::ColorMap;
pub use error::TgaError;
pub use header::{Bpp, DataType, ImageOrigin, TgaHeader};
pub use point::Point;

mod color_map;
mod error;
mod footer;
mod header;
mod pixels;
mod point;
mod tga;
