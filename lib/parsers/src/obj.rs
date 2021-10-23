pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
    }

    /// Get a reference to the vertex's normal.
    pub fn normal(&self) -> &[f32; 3] {
        &self.normal
    }

    /// Get a reference to the vertex's position.
    pub fn position(&self) -> &[f32; 3] {
        &self.position
    }
}
