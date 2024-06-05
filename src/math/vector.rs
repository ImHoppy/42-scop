#[repr(C)]
#[derive(Copy, PartialEq, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, PartialEq, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, PartialEq, Clone, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

macro_rules! impl_vector {
    // Implement the `VectorN` struct. This macro is used to implement the `Vector2`, `Vector3`, and `Vector4` structs.
    (
		$VectorN:ident { $($field:ident),+ },
		$VectorNFieldsCount:literal,
		$constructor:ident
	) => {
        impl $VectorN {
            #[inline]
            pub const fn new($($field: f32),+) -> Self {
                $VectorN { $($field: $field),+ }
            }

            #[inline]
            pub fn normalize_to(self, magnitude: f32) -> Self {
                self * (magnitude / self.magnitude())
            }

            #[inline]
            pub fn normalize(self) -> Self {
                self.normalize_to(1.0)
            }

            pub fn magnitude(self) -> f32
            {
                let sum = $(self.$field * self.$field +)+ 0.0;
                sum.sqrt()
            }

            pub fn dot(self, other: $VectorN) -> f32 {
                let mut sum = 0.0;
                $(sum += self.$field * other.$field;)+
                sum
            }
        }

        impl std::ops::Mul<f32> for $VectorN {
            type Output = $VectorN;

            fn mul(self, scalar: f32) -> Self::Output {
                $VectorN {
                    $($field: self.$field * scalar),+
                }
            }
        }

        impl std::ops::Sub<$VectorN> for $VectorN {
            type Output = $VectorN;

            fn sub(self, other: $VectorN) -> Self::Output {
                $VectorN {
                    $($field: self.$field - other.$field),+
                }
            }
        }


		#[inline]
		pub const fn $constructor($($field: f32),+) -> $VectorN {
			$VectorN::new($($field),+)
		}
    };
}

impl Vector3 {
    /// Returns the cross product of the vector and `other`.
    #[inline]
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl_vector!(Vector2 { x, y }, 2, vec2);
impl_vector!(Vector3 { x, y, z }, 3, vec3);
impl_vector!(Vector4 { x, y, z, w }, 4, vec4);
