
pub use header::{TgaHeader, Bpp, DataType, ImageOrigin};
pub use error::TgaError;
pub use color_map::ColorMap;

mod size;
mod tga;
mod header;
mod error;
mod color_map;
