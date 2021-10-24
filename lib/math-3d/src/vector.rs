#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    rep: gfx_maths::Vec3,
}

pub struct Vector2 {
    rep: gfx_maths::Vec2,
}

impl Vector2 {
    pub fn new(rep: gfx_maths::Vec2) -> Self {
        Self { rep }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            rep: gfx_maths::Vec3 { x, y, z },
        }
    }

    pub fn dot(self, rhs: Vector3) -> f32 {
        self.rep.dot(rhs.rep)
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Self {
            rep: self.rep.cross(rhs.rep),
        }
    }
}

impl std::ops::Deref for Vector3 {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        // # Safety not sure but this implementation is temporary as I move away from this lib
        unsafe { std::slice::from_raw_parts((&self.rep.x) as *const f32, 3) }
    }
}

impl std::ops::DerefMut for Vector3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // # Safety not sure but this implementation is temporary as I move away from this lib
        unsafe { std::slice::from_raw_parts_mut((&mut self.rep.x) as *mut f32, 3) }
    }
}

impl std::ops::Deref for Vector2 {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        // # Safety not sure but this implementation is temporary as I move away from this lib
        unsafe { std::slice::from_raw_parts((&self.rep.x) as *const f32, 2) }
    }
}

impl std::ops::DerefMut for Vector2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // # Safety not sure but this implementation is temporary as I move away from this lib
        unsafe { std::slice::from_raw_parts_mut((&mut self.rep.x) as *mut f32, 2) }
    }
}
