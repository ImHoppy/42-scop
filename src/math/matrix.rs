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
impl std::ops::Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, scalar: f32) -> Self::Output {
        Matrix4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl std::ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, scalar: Matrix4) -> Self::Output {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::from_cols(
            a*scalar[0][0] + b*scalar[0][1] + c*scalar[0][2] + d*scalar[0][3],
            a*scalar[1][0] + b*scalar[1][1] + c*scalar[1][2] + d*scalar[1][3],
            a*scalar[2][0] + b*scalar[2][1] + c*scalar[2][2] + d*scalar[2][3],
            a*scalar[3][0] + b*scalar[3][1] + c*scalar[3][2] + d*scalar[3][3],
        )
    }
}

impl std::ops::Index<usize> for Matrix4 {
    type Output = Vector4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Matrix4"),
        }
    }
}
impl std::ops::IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds for Matrix4"),
        }
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

    /// Creates a matrix that rotates around the x-axis. Theta is in radians.
    pub fn from_angle_x(theta: f32) -> Matrix4 {
        let c = f32::cos(theta);
        let s = f32::sin(theta);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, c, s, 0.0,
            0.0, -s, c, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
    /// Creates a matrix that rotates around the y-axis. Theta is in radians.
    pub fn from_angle_y(theta: f32) -> Matrix4 {
        let c = f32::cos(theta);
        let s = f32::sin(theta);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c, 0.0, -s, 0.0,
            0.0, 1.0, 0.0, 0.0,
            s, 0.0, c, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn from_translation(translation: Vector3) -> Matrix4 {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            translation.x, translation.y, translation.z, 1.0,
        )
    }
}
