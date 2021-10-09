use iced::canvas::Path;

use crate::data::TimeTable;
use crate::{Pose, WithPath, RED_B, YELLOW_C};
use crate::{Spatial, WithSpatial};
use crate::{Visual, WithVisual};

#[derive(Debug)]
pub struct Car {
    spatial: Spatial,
    visual: Visual,
    id: usize,
}

impl Default for Car {
    fn default() -> Self {
        Self {
            visual: Visual::default()
                .with_fill_color(YELLOW_C)
                .with_stroke_color(RED_B),
            spatial: Spatial::default().with_width(2.0).with_height(4.0),
            id: 0,
        }
    }
}

impl Car {
    pub fn goto_time(&mut self, data: &TimeTable<Pose>, time_now: f32) {
        data.get_at_time(self.id, time_now)
            .map(|pose| self.set_pose(pose));
    }

    // pub fn draw(&self, bounds: Rectangle, scale: (f32, f32), origin: Point) -> Geometry {
    //     // let prim = Primitive::Svg {
    //     //     handle: svg::Handle::from_path(format!(
    //     //         "{}/resources/car.svg",
    //     //         env!("CARGO_MANIFEST_DIR")
    //     //     )),
    //     //     bounds,
    //     // };

    //     // let prim = Primitive::Image {
    //     //     handle: image::Handle::from_path(format!(
    //     //         "{}/resources/car.png",
    //     //         env!("CARGO_MANIFEST_DIR")
    //     //     )),
    //     //     bounds,
    //     // };
    //     // Geometry(prim) // This doesn't work
    // }

    pub fn path(&self) -> Path {
        let top_left = self.size() / -2.0;
        Path::rectangle(top_left.into(), self.size().into())
    }
}

impl WithPath for Car {
    fn path(&self) -> Path {
        let top_left = self.size() / -2.0;
        Path::rectangle(top_left.into(), self.size().into())
    }
}

impl WithSpatial for Car {
    fn get(&self) -> &Spatial {
        &self.spatial
    }
    fn get_mut(&mut self) -> &mut Spatial {
        &mut self.spatial
    }
}

impl WithVisual for Car {
    fn get(&self) -> &crate::Visual {
        &self.visual
    }
    fn get_mut(&mut self) -> &mut Visual {
        &mut self.visual
    }
}
