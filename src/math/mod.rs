// pub use angle;
pub use matrix::Matrix4;
pub use vector::{vec2, vec3, vec4, Vector2, Vector3, Vector4};

pub type Vec2 = Vector2;
pub type Vec3 = Vector3;
pub type Vec4 = Vector4;

mod angle;
mod matrix;
mod vector;
