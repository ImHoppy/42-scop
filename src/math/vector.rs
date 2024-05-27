#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

macro_rules! impl_vector {
    // Implement the `VectorN` struct. This macro is used to implement the `Vector2`, `Vector3`, and `Vector4` structs.
    (
		$VectorN:ident { $($field:ident),+ },
		$VectorNFieldsCount:literal,
		$constructor:ident
	) => {
        impl<T> $VectorN<T> {
            #[inline]
            pub const fn new($($field: T),+) -> Self {
                $VectorN { $($field: $field),+ }
            }
        }

		#[inline]
		pub const fn $constructor<T>($($field: T),+) -> $VectorN<T> {
			$VectorN::new($($field),+)
		}
    };
}

impl_vector!(Vector2 { x, y }, 2, vec2);
impl_vector!(Vector3 { x, y, z }, 3, vec3);
impl_vector!(Vector4 { x, y, z, w }, 4, vec4);
