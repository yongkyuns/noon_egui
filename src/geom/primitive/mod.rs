pub mod axis;
pub mod ellipse;
pub mod grid;
pub mod line;
pub mod object;
pub mod rectangle;
pub mod text;

pub use ellipse::{ellipse, Circle, Ellipse};
pub use object::Car;

pub struct Step {
    pub x: f32,
    pub y: f32,
}

impl Default for Step {
    fn default() -> Self {
        Step { x: 5.0, y: 5.0 }
    }
}
