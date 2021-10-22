#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    values: [f32; 3],
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { values: [x, y, z] }
    }

    pub fn dot(self, rhs: Vector3) -> f32 {
        self.values.into_iter().zip(rhs).map(|(l, r)| l * r).sum()
    }
}

/// # Consuming IntoIterator
///
/// This would allow an `Iterator<Item = Vector3>` to be flattened into `Iterator<Item = f32>`
impl IntoIterator for Vector3 {
    type Item = f32;
    type IntoIter = std::array::IntoIter<f32, 3>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

pub trait ArrayWrapper<T>: Sized {
    fn wrapped_slice_as_base_slice(slice: &[Self]) -> &[T];
}

impl ArrayWrapper<f32> for Vector3 {
    fn wrapped_slice_as_base_slice(slice: &[Self]) -> &[f32] {
        let ptr = slice.as_ptr() as *const f32;
        let len = slice.len();

        unsafe { std::slice::from_raw_parts(ptr, len * 3) }
    }
}

/// # Derefs as &[f32]
///
/// This is adds all the slice functionality (iteration, etc), in particular
/// the ability to be passed as a pointer to graphics APIs
impl std::ops::Deref for Vector3 {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

/// # Derefs as &mut [f32]
///
/// As with the Deref, but allows mutation
impl std::ops::DerefMut for Vector3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
