pub mod color;

pub use self::color::{Color, DEFAULT_FILL_COLOR, DEFAULT_STROKE_COLOR};

pub const DEFAULT_STROKE_WEIGHT: f32 = 3.0;
pub const DEFAULT_TEXT_STROKE_WEIGHT: f32 = 1.0;

pub const DEFAULT_FLATTEN_TOLERANCE: f32 = 0.01;
pub const DEFAULT_RUNTIME: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
pub struct Fill {
    color: Color,
}

impl From<Color> for Fill {
    fn from(color: Color) -> Self {
        Fill { color }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            color: DEFAULT_FILL_COLOR,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    color: Color,
    weight: f32,
}

impl From<Color> for Stroke {
    fn from(color: Color) -> Self {
        Stroke {
            color,
            weight: DEFAULT_STROKE_WEIGHT,
        }
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: DEFAULT_STROKE_COLOR,
            weight: DEFAULT_STROKE_WEIGHT,
        }
    }
}

#[derive(Debug)]
pub enum Visual {
    Fill(Fill),
    Stroke(Stroke),
    Both((Fill, Stroke)),
}

impl Default for Visual {
    fn default() -> Self {
        Visual::Both((Fill::default(), Stroke::default()))
    }
}

impl Visual {
    pub fn stroke() -> Self {
        Visual::Stroke(Stroke::default())
    }
    pub fn fill() -> Self {
        Visual::Fill(Fill::default())
    }
}

pub trait WithVisual: Sized {
    fn get(&self) -> &Visual;
    fn get_mut(&mut self) -> &mut Visual;
    fn set_fill_color(&mut self, color: Color) {
        match self.get_mut() {
            Visual::Fill(fill) => fill.color = color,
            Visual::Stroke(stroke) => *self.get_mut() = Visual::Both((Fill::from(color), *stroke)),
            Visual::Both((fill, _)) => fill.color = color,
        }
    }
    fn set_stroke_color(&mut self, color: Color) {
        match self.get_mut() {
            Visual::Fill(fill) => *self.get_mut() = Visual::Both((*fill, Stroke::from(color))),
            Visual::Stroke(stroke) => stroke.color = color,
            Visual::Both((_, stroke)) => stroke.color = color,
        }
    }
    fn set_stroke_weight(&mut self, weight: f32) {
        match self.get_mut() {
            Visual::Stroke(stroke) => stroke.weight = weight,
            Visual::Both((_, stroke)) => stroke.weight = weight,
            _ => (),
        }
    }
    fn with_color(self, color: Color) -> Self {
        self.with_stroke_color(color).with_fill_color(color)
    }
    // fn with_width(mut self, width: f32) -> Self {
    //     self.set_stroke_width(width);
    //     self
    // }
    fn with_fill_color(mut self, color: Color) -> Self {
        self.set_fill_color(color);
        self
    }
    fn with_stroke_color(mut self, color: Color) -> Self {
        self.set_stroke_color(color);
        self
    }
    fn with_stroke_width(mut self, weight: f32) -> Self {
        self.set_stroke_weight(weight);
        self
    }
    fn fill(&self) -> Option<Fill> {
        match self.get() {
            Visual::Fill(fill) => Some(*fill),
            Visual::Both((fill, _)) => Some(*fill),
            Visual::Stroke(_) => None,
        }
    }
    fn stroke(&self) -> Option<Stroke> {
        match self.get() {
            Visual::Stroke(stroke) => Some(*stroke),
            Visual::Both((_, stroke)) => Some(*stroke),
            Visual::Fill(_) => None,
        }
    }
}

impl WithVisual for Visual {
    fn get(&self) -> &Visual {
        self
    }
    fn get_mut(&mut self) -> &mut Visual {
        self
    }
}

#[cfg(test)]
mod appearance_test {
    use super::*;
    #[test]
    fn check_default() {
        let _ = Visual::default();
        let _ = Visual::stroke();
        let _ = Visual::fill();
    }
}
