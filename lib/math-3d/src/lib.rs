pub mod vector;
pub use vector::*;

pub struct Matrix4 {
    values: [f32; 16],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            values: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            ],
        }
    }
}

/// # Derefs as &[f32]
///
/// This is adds all the slice functionality (iteration, etc), in particular
/// the ability to be passed as a pointer to graphics APIs
impl std::ops::Deref for Matrix4 {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
