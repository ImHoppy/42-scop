use std::ops::{Add, Sub, Mul, Div, Index};


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Size {
    /// The width.
    pub width: u32,

    /// The height.
    pub height: u32,
}

impl Size {
	/// Default constructor
	pub const fn new(width: u32, height: u32) -> Self {
		Size { width, height }
	}

	pub const fn zero() -> Self {
		Size { width: 0, height: 0, }
	}
}

impl Add for Size {
	type Output = Size;

	fn add(self, other: Size) -> Size {
		Size::new(self.width + other.width, self.height + other.height)
	}
}

impl Sub for Size {
	type Output = Size;

	fn sub(self, other: Size) -> Size {
		Size::new(self.width - other.width, self.height - other.height)
	}
}

impl Mul for Size {
	type Output = Size;

	fn mul(self, other: Size) -> Size {
		Size::new(self.width * other.width, self.height * other.height)
	}
}

impl Div for Size {
	type Output = Size;

	fn div(self, other: Size) -> Size {
		Size::new(self.width / other.width, self.height / other.height)
	}
}

impl Index<usize> for Size {
	type Output = u32;

	fn index(&self, index: usize) -> &u32 {
        match index {
            0 => &self.width,
            1 => &self.height,
            _ => panic!("index out of bounds: {}", index),
        }
	}
}
