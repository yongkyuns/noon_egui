use crate::{Spatial, WithSpatial};
use crate::{Visual, WithVisual};

#[derive(Debug)]
pub struct Circle(Ellipse);

impl Circle {
    pub fn with_radius(self, radius: f32) -> Self {
        circle().with_width(2.0 * radius).with_height(2.0 * radius)
    }
    pub fn with_width(mut self, width: f32) -> Self {
        WithSpatial::get_mut(&mut self).size.width = width;
        WithSpatial::get_mut(&mut self).size.height = width;
        self
    }
    pub fn with_height(mut self, height: f32) -> Self {
        WithSpatial::get_mut(&mut self).size.height = height;
        WithSpatial::get_mut(&mut self).size.width = height;
        self
    }
    pub fn radius(&self) -> f32 {
        self.size().width / 2.0
    }
}

impl WithSpatial for Circle {
    fn get(&self) -> &Spatial {
        WithSpatial::get(&self.0)
    }
    fn get_mut(&mut self) -> &mut Spatial {
        WithSpatial::get_mut(&mut self.0)
    }
}

impl WithVisual for Circle {
    fn get(&self) -> &Visual {
        WithVisual::get(&self.0)
    }
    fn get_mut(&mut self) -> &mut Visual {
        WithVisual::get_mut(&mut self.0)
    }
}

// impl WithPath for Circle {
//     fn path(&self) -> Path {
//         Path::circle(Point::ORIGIN, self.radius())
//     }
// }

pub fn circle() -> Circle {
    Circle(ellipse())
}

#[derive(Debug)]
pub struct Ellipse {
    spatial: Spatial,
    visual: Visual,
}

impl Ellipse {
    pub fn with_radii(self, radii: (f32, f32)) -> Self {
        ellipse()
            .with_width(2.0 * radii.0)
            .with_height(2.0 * radii.1)
    }
    pub fn with_width(mut self, width: f32) -> Self {
        WithSpatial::get_mut(&mut self).size.width = width;
        self
    }
    pub fn with_height(mut self, height: f32) -> Self {
        WithSpatial::get_mut(&mut self).size.height = height;
        self
    }
    pub fn radii(&self) -> (f32, f32) {
        (self.size().width / 2.0, self.size().height / 2.0)
    }
}

// impl WithPath for Ellipse {
//     fn path(&self) -> Path {
//         use iced_graphics::canvas::path::arc::Elliptical;
//         Path::new(|builder| {
//             builder.ellipse(Elliptical {
//                 center: Point::ORIGIN,
//                 radii: iced::Vector::new(self.radii().0, self.radii().1),
//                 rotation: self.angle(),
//                 start_angle: 0.0,
//                 end_angle: std::f32::consts::PI,
//             });
//         })
//     }
// }

impl WithSpatial for Ellipse {
    fn get(&self) -> &Spatial {
        &self.spatial
    }
    fn get_mut(&mut self) -> &mut Spatial {
        &mut self.spatial
    }
}

impl WithVisual for Ellipse {
    fn get(&self) -> &Visual {
        &self.visual
    }
    fn get_mut(&mut self) -> &mut Visual {
        &mut self.visual
    }
}

pub fn ellipse() -> Ellipse {
    Ellipse {
        spatial: Spatial::default(),
        visual: Visual::default(),
    }
}

#[cfg(test)]
mod test_ellipse {
    use super::*;
    #[test]
    fn check_ellipse() {
        let c = circle().with_width(20.0);
        assert_eq!(10.0, c.radius());
        assert_eq!(20.0, c.height());

        let e = ellipse().with_radii((20.0, 10.0));
        assert_eq!(20.0, e.radii().0);
        assert_eq!(10.0, e.radii().1);

        // dbg!(c);
        // dbg!(e);
    }
}
