use super::Step;
use crate::geom::path::WithPath;
use crate::{pt, Path, Spatial, Visual, WithSpatial, WithVisual, DARKER_GRAY};

use iced::canvas::Stroke;

pub struct Grid {
    visual: Visual,
    spatial: Spatial,
    pub step: Step,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            visual: Visual::Stroke(Stroke::default().with_color(DARKER_GRAY)),
            spatial: Spatial::default(),
            step: Step::default(),
        }
    }
}

impl WithSpatial for Grid {
    fn get(&self) -> &Spatial {
        WithSpatial::get(&self.spatial)
    }
    fn get_mut(&mut self) -> &mut Spatial {
        WithSpatial::get_mut(&mut self.spatial)
    }
}

impl WithVisual for Grid {
    fn get(&self) -> &Visual {
        WithVisual::get(&self.visual)
    }
    fn get_mut(&mut self) -> &mut Visual {
        WithVisual::get_mut(&mut self.visual)
    }
}

impl WithPath for Grid {
    fn path(&self) -> Path {
        Path::new(|builder| {
            let w = self.width() / 2.0;
            let h = self.height() / 2.0;
            let p = pt(self.position().x, -self.position().y);

            if w / self.step.x < 50.0 {
                let step_by = || (0..).map(|i| i as f32 * self.step.x);
                let r_iter = step_by().map(|f| f - p.x).take_while(|&f| f < w);
                let l_iter = step_by().map(|f| -f - p.x).take_while(|&f| f > -w);
                let x_iter = r_iter.chain(l_iter);
                for x in x_iter {
                    builder.move_to(pt(x, h).into());
                    builder.line_to(pt(x, -h).into());
                }
            }

            if h / self.step.y < 50.0 {
                let step_by = || (0..).map(|i| i as f32 * self.step.y);
                let u_iter = step_by().map(|f| f - p.y).take_while(|&f| f < h);
                let d_iter = step_by().map(|f| -f - p.y).take_while(|&f| f > -h);
                let y_iter = u_iter.chain(d_iter);
                for y in y_iter {
                    builder.move_to(pt(w, y).into());
                    builder.line_to(pt(-w, y).into());
                }
            }
        })
    }
}
