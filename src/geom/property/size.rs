use crate::geom::{pt, Point};
use nannou::geom::Rect;
use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size<T = f32> {
    pub width: T,
    pub height: T,
}

impl Default for Size {
    fn default() -> Self {
        Size::UNIT
    }
}

impl Size {
    pub const ZERO: Size = Size {
        width: 0.0,
        height: 0.0,
    };
    pub const UNIT: Size = Size {
        width: 1.0,
        height: 1.0,
    };
}

impl Mul<Size> for f32 {
    type Output = Size;
    fn mul(self, size: Size) -> Self::Output {
        Size {
            width: self * size.width,
            height: self * size.height,
        }
    }
}

impl Div<f32> for Size {
    type Output = Self;
    fn div(self, den: f32) -> Self::Output {
        Self {
            width: self.width / den,
            height: self.height / den,
        }
    }
}

impl Into<Rect> for Size {
    fn into(self) -> Rect {
        Rect::from_w_h(self.width, self.height)
    }
}

impl Into<Size> for Rect {
    fn into(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl Into<Point> for Size {
    fn into(self) -> Point {
        pt(self.width, self.height)
    }
}

pub fn size(width: f32, height: f32) -> Size {
    Size { width, height }
}
