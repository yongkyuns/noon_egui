use crate::geom::{pt, Point};

#[derive(Debug, Clone, Copy, Default)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub phi: f32,
}

impl Pose {
    pub fn new(x: f32, y: f32, phi: f32) -> Self {
        Self { x, y, phi }
    }
    pub fn position(&self) -> Point {
        pt(self.x, self.y)
    }
    pub fn angle(&self) -> f32 {
        self.phi
    }
}
