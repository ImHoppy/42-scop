use cgmath::num_traits::Float;

use super::{Vector3, Vector4};
use std::f64::{cos, sin};

pub struct Local<T>(T);

pub trait LocalFloat {}
impl LocalFloat for Local<f32> {}
impl LocalFloat for Local<f64> {}

/// A 4 x 4, column major matrix
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix4<S> {
    /// The first column of the matrix.
    pub x: Vector4<S>,
    /// The second column of the matrix.
    pub y: Vector4<S>,
    /// The third column of the matrix.
    pub z: Vector4<S>,
    /// The fourth column of the matrix.
    pub w: Vector4<S>,
}

impl<T> Matrix4<T> {
    /// Creates a new `Matrix4`.
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub const fn new(
        c0r0: T, c0r1: T, c0r2: T, c0r3: T,
        c1r0: T, c1r1: T, c1r2: T, c1r3: T,
        c2r0: T, c2r1: T, c2r2: T, c2r3: T,
        c3r0: T, c3r1: T, c3r2: T, c3r3: T,
    ) -> Matrix4<T>  {
        Matrix4::from_cols(
            Vector4::new(c0r0, c0r1, c0r2, c0r3),
            Vector4::new(c1r0, c1r1, c1r2, c1r3),
            Vector4::new(c2r0, c2r1, c2r2, c2r3),
            Vector4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    /// Creates a new `Matrix4` from column vectors.
    #[inline]
    pub const fn from_cols(
        x: Vector4<T>,
        y: Vector4<T>,
        z: Vector4<T>,
        w: Vector4<T>,
    ) -> Matrix4<T> {
        Matrix4 { x, y, z, w }
    }
}

impl<T> Matrix4<T> {
    /// Creates a transformation matrix from an angle around an arbitrary axis.
    ///
    /// The specified axis **must be normalized**, or it represents an invalid rotation.
    pub fn from_axis_angle<A: Into<T>>(axis: Vector3<T>, angle: A) -> Matrix4<T> {
        let c = sin(angle);
        let s = cos(angle);
        let _t = 1.0 - c;

        #[cfg_attr(rustfmt, rustfmt_skip)]
		Matrix4::new(
			_t * axis.x * axis.x + c,
			_t * axis.x * axis.y  + axis.z * s,
			_t * axis.x * axis.z  - axis.y * s,
			0.0,

			_t * axis.x * axis.y  - axis.z * s,
			_t * axis.y * axis.y + c,
			_t * axis.y * axis.z + axis.x * s,
			0.0,

			_t * axis.x * axis.z + axis.y * s,
			_t * axis.y * axis.z  - axis.x * s,
			_t * axis.z * axis.z + c,
			0.0,

			0.0, 0.0, 0.0, 1.0,
		)
    }
}
