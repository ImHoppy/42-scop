use super::{Deg, Matrix4, Rad};

/// Create a perspective projection matrix.
pub fn perspective(fovy: Deg, aspect: f32, near: f32, far: f32) -> Matrix4 {
    let fovy_rad: Rad = fovy.into();

    let f = (fovy_rad / 2.0).tan().recip();

    let c0r0 = f / aspect;
    let c0r1 = 0.0;
    let c0r2 = 0.0;
    let c0r3 = 0.0;

    let c1r0 = 0.0;
    let c1r1 = f * -1.0; // negate the value to invert the Y axis for Vulkan
    let c1r2 = 0.0;
    let c1r3 = 0.0;

    let c2r0 = 0.0;
    let c2r1 = 0.0;
    let c2r2 = (far + near) / (near - far);
    let c2r3 = -1.0;

    let c3r0 = 0.0;
    let c3r1 = 0.0;
    let c3r2 = (2.0 * far * near) / (near - far);
    let c3r3 = 0.0;

    #[cfg_attr(rustfmt, rustfmt_skip)]
	Matrix4::new(
		c0r0, c0r1, c0r2, c0r3,
		c1r0, c1r1, c1r2, c1r3,
		c2r0, c2r1, c2r2, c2r3,
		c3r0, c3r1, c3r2, c3r3,
	)
}
