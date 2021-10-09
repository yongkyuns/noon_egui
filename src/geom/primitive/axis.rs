use super::Step;
use crate::appearance::color::DARK_GRAY;
use crate::{Spatial, Visual, WithSpatial, WithVisual};

pub struct Axis {
    visual: Visual,
    spatial: Spatial,
    pub step: Step,
    tick_size: f32,
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            visual: Visual::stroke()
                .with_color(DARK_GRAY)
                .with_stroke_width(1.0),
            spatial: Spatial::default(),
            step: Step::default(),
            tick_size: 1.0,
        }
    }
}

impl WithSpatial for Axis {
    fn get(&self) -> &Spatial {
        WithSpatial::get(&self.spatial)
    }
    fn get_mut(&mut self) -> &mut Spatial {
        WithSpatial::get_mut(&mut self.spatial)
    }
}

impl WithVisual for Axis {
    fn get(&self) -> &Visual {
        WithVisual::get(&self.visual)
    }
    fn get_mut(&mut self) -> &mut Visual {
        WithVisual::get_mut(&mut self.visual)
    }
}

// impl WithPath for Axis {
//     fn path(&self) -> Path {
//         Path::new(|builder| {
//             let t = self.tick_size / 2.0;
//             let w = self.width() / 2.0;
//             let h = self.height() / 2.0;
//             let p = pt(self.position().x, -self.position().y);

//             // Horizontal axis
//             builder.move_to(pt(w, -p.y).into());
//             builder.line_to(pt(-w, -p.y).into());
//             // Vertical axis
//             builder.move_to(pt(-p.x, h).into());
//             builder.line_to(pt(-p.x, -h).into());

//             // Draw only if zoom is close enough to see the grid
//             if w / self.step.x < 50.0 {
//                 let step_by = || (0..).map(|i| i as f32 * self.step.x);
//                 let r_iter = step_by().map(|f| f - p.x).take_while(|&f| f < w);
//                 let l_iter = step_by().map(|f| -f - p.x).take_while(|&f| f > -w);
//                 let x_iter = r_iter.chain(l_iter);
//                 for x in x_iter {
//                     builder.move_to((pt(x, t - p.y)).into());
//                     builder.line_to((pt(x, -t - p.y)).into());
//                 }
//             }

//             if h / self.step.y < 50.0 {
//                 let step_by = || (0..).map(|i| i as f32 * self.step.y);
//                 let u_iter = step_by().map(|f| f - p.y).take_while(|&f| f < h);
//                 let d_iter = step_by().map(|f| -f - p.y).take_while(|&f| f > -h);
//                 let y_iter = u_iter.chain(d_iter);
//                 for y in y_iter {
//                     builder.move_to((pt(t - p.x, y)).into());
//                     builder.line_to((pt(-t - p.x, y)).into());
//                 }
//             }
//         })
//     }
// }
