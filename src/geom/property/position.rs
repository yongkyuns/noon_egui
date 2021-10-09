pub use nannou::geom::pt2 as pt;
pub use nannou::geom::Point2 as Point;
pub use nannou::geom::Vec2 as Vector;

pub use nannou::lyon::geom::euclid::UnknownUnit;

// impl Into<nannou::lyon::geom::euclid::Point2D<f32, UnknownUnit>> for Point {
//     fn into(self) -> nannou::lyon::geom::euclid::Point2D<f32, UnknownUnit> {}
// }

// use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

// pub type Vector<T = f32> = Point<T>;

// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub struct Point<T = f32> {
//     pub x: T,
//     pub y: T,
// }

// impl Default for Point {
//     fn default() -> Self {
//         Point::ZERO
//     }
// }

// impl Point {
//     pub const ZERO: Point = Point { x: 0.0, y: 0.0 };
//     pub const ONE: Point = Point { x: 1.0, y: 1.0 };
//     pub fn new(x: f32, y: f32) -> Self {
//         Self { x, y }
//     }
// }

// pub fn pt<T>(x: T, y: T) -> Point<T> {
//     Point { x, y }
// }

// impl Neg for Point {
//     type Output = Self;
//     fn neg(self) -> Self::Output {
//         -1.0 * self
//     }
// }

// impl Add for Point {
//     type Output = Self;
//     fn add(self, other: Self) -> Self::Output {
//         Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// impl AddAssign for Point {
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// impl Sub for Point {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self::Output {
//         Self {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// impl SubAssign for Point {
//     fn sub_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// impl Mul<f32> for Point {
//     type Output = Self;
//     fn mul(self, scale: f32) -> Self::Output {
//         Self {
//             x: self.x * scale,
//             y: self.y * scale,
//         }
//     }
// }

// impl Mul<Point> for f32 {
//     type Output = Point;
//     fn mul(self, point: Point) -> Self::Output {
//         Point {
//             x: self * point.x,
//             y: self * point.y,
//         }
//     }
// }

// impl Div<f32> for Point {
//     type Output = Self;
//     fn div(self, den: f32) -> Self::Output {
//         Self {
//             x: self.x / den,
//             y: self.y / den,
//         }
//     }
// }

// impl Into<nannou::geom::Point2> for Point {
//     fn into(self) -> nannou::geom::Point2 {
//         nannou::geom::pt2(self.x, self.y)
//     }
// }

// impl Into<Point> for nannou::geom::Point2 {
//     fn into(self) -> Point {
//         Point {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

// impl Into<nannou::geom::Vector2> for Point {
//     fn into(self) -> nannou::geom::Vector2 {
//         nannou::geom::vec2(self.x, self.y)
//     }
// }

// impl From<nannou::geom::Vector2> for Point {
//     fn from(v: nannou::geom::Vector2) -> Self {
//         Self { x: v.x, y: v.y }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn check_point() {
//         let mut p = pt(5.0, 5.0);
//         let p2 = pt(3.0, 3.0);
//         p += p2;

//         assert_eq!(1, 1);
//     }
// }
