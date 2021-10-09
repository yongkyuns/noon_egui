pub mod path;
pub mod primitive;
pub mod property;

pub use property::orientation::Angle;
pub use property::pose::Pose;
pub use property::position::{pt, Point, Vector};
pub use property::rect::Rect;
pub use property::size::Size;

#[derive(Debug)]
pub struct Spatial {
    position: Point,
    orientation: Angle,
    size: Size,
}

impl WithSpatial for Spatial {
    fn get(&self) -> &Spatial {
        self
    }
    fn get_mut(&mut self) -> &mut Spatial {
        self
    }
}

impl Default for Spatial {
    fn default() -> Self {
        Self {
            size: Size::UNIT,
            position: Point::ZERO,
            orientation: 0.0,
        }
    }
}

pub trait WithSpatial: Sized {
    fn get_mut(&mut self) -> &mut Spatial;
    fn get(&self) -> &Spatial;

    fn at(self, position: Point) -> Self {
        self.move_to(position)
    }
    fn move_to(mut self, position: Point) -> Self {
        self.get_mut().position = position;
        self
    }
    fn move_by(mut self, position: Point) -> Self {
        self.get_mut().position += position;
        self
    }
    fn with_pose(mut self, pose: Pose) -> Self {
        self.get_mut().position = pt(pose.x, pose.y);
        self.get_mut().orientation = pose.angle();
        self
    }
    fn set_pose(&mut self, pose: Pose) {
        self.get_mut().position = pt(pose.x, pose.y);
        self.get_mut().orientation = pose.angle();
    }
    fn with_angle(self, angle: Angle) -> Self {
        self.rotate_to(angle)
    }
    fn rotate_to(mut self, angle: Angle) -> Self {
        self.get_mut().orientation = angle;
        self
    }
    fn rotate_by(mut self, angle: Angle) -> Self {
        self.get_mut().orientation += angle;
        self
    }
    fn with_size(mut self, size: Size) -> Self {
        self.get_mut().size = size;
        self
    }
    fn with_width(mut self, width: f32) -> Self {
        self.get_mut().size.width = width;
        self
    }
    fn with_height(mut self, height: f32) -> Self {
        self.get_mut().size.height = height;
        self
    }

    fn position(&self) -> Point {
        self.get().position
    }
    fn angle(&self) -> Angle {
        self.get().orientation
    }
    fn size(&self) -> Size {
        self.get().size
    }
    fn width(&self) -> f32 {
        self.get().size.width
    }
    fn height(&self) -> f32 {
        self.get().size.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::pt;
    #[test]
    fn test_case() {
        let sp: Spatial = Default::default();
        dbg!(sp.move_to(pt(10.0, 5.0)));
        // sp.width(10.0);
        // dbg!(&sp);

        // assert_eq!(1, 1);
    }
}
