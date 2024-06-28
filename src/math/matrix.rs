use super::{Vector3, Vector4};
use std::f32;

/// A 4 x 4, column major matrix
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix4 {
    /// The first column of the matrix.
    pub x: Vector4,
    /// The second column of the matrix.
    pub y: Vector4,
    /// The third column of the matrix.
    pub z: Vector4,
    /// The fourth column of the matrix.
    pub w: Vector4,
}

impl Matrix4 {
    /// Creates a new `Matrix4`.
    #[inline]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub const fn new(
        c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32,
        c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32,
        c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32,
        c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32,
    ) -> Matrix4  {
        Matrix4::from_cols(
            Vector4::new(c0r0, c0r1, c0r2, c0r3),
            Vector4::new(c1r0, c1r1, c1r2, c1r3),
            Vector4::new(c2r0, c2r1, c2r2, c2r3),
            Vector4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    /// Creates a new `Matrix4` from column vectors.
    #[inline]
    pub const fn from_cols(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Matrix4 {
        Matrix4 { x, y, z, w }
    }
}

impl Matrix4 {
    /// Creates a transformation matrix from an angle around an arbitrary axis.
    ///
    /// The specified axis **must be normalized**, or it represents an invalid rotation.
    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Matrix4 {
        let c = f32::sin(angle);
        let s = f32::cos(angle);
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

    /// Create a homogeneous transformation matrix that will cause a vector to point at
    /// `dir`, using `up` for orientation.
    pub fn look_to_rh(eye: Vector3, dir: Vector3, up: Vector3) -> Matrix4 {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            s.x, u.x, -f.x, 0.0,
            s.y, u.y, -f.y, 0.0,
            s.z, u.z, -f.z, 0.0,
            -eye.dot(s), -eye.dot(u), eye.dot(f), 1.0,
        )
    }

    pub fn look_at_rh(eye: Vector3, center: Vector3, up: Vector3) -> Matrix4 {
        Matrix4::look_to_rh(eye, center - eye, up)
    }
}
