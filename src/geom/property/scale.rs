/// Scale for x and y axis
#[derive(Debug, Clone, Copy)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

impl Scale {
    /// Minimum scale between x and y
    pub fn min(&self) -> f32 {
        if self.x < self.y {
            self.x
        } else {
            self.y
        }
    }
    /// Maximum scale between x and y
    pub fn max(&self) -> f32 {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale { x: 1.0, y: 1.0 }
    }
}
