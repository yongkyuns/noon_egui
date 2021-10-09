use crate::appearance::Visual;
use crate::appearance::WithVisual;
use crate::Spatial;
use crate::WithSpatial;

pub struct Text<'a> {
    pub(crate) raw: nannou::text::Text<'a>,
    spatial: Spatial,
    visual: Visual,
}

impl<'a> WithSpatial for Text<'a> {
    fn get(&self) -> &Spatial {
        &self.spatial
    }
    fn get_mut(&mut self) -> &mut Spatial {
        &mut self.spatial
    }
}

impl<'a> WithVisual for Text<'a> {
    fn get(&self) -> &Visual {
        &self.visual
    }
    fn get_mut(&mut self) -> &mut Visual {
        &mut self.visual
    }
}

// impl Text {
//     pub fn at(mut self, position: Point) -> Self {
//         self.raw.position = position.into();
//         self
//     }
//     pub fn with_color(mut self, color: Color) -> Self {
//         self.raw.color = color;
//         self
//     }
//     pub fn with_font_size(mut self, size: f32) -> Self {
//         self.raw.size = size;
//         self
//     }
//     pub fn with_font(mut self, font: Font) -> Self {
//         self.raw.font = font;
//         self
//     }
//     pub fn scale_by(mut self, scale: f32) -> Self {
//         self.raw.size *= scale;
//         self
//     }
//     pub fn draw(self, size: Size) -> Geometry {
//         let mut frame = Frame::new(size);
//         frame.fill_text(self.raw);
//         frame.into_geometry()
//     }
//     pub fn position(&self) -> Point {
//         self.raw.position.into()
//     }
// }

// impl Into<iced::canvas::Text> for Text {
//     fn into(self) -> iced::canvas::Text {
//         self.raw
//     }
// }

// impl From<String> for Text {
//     fn from(content: String) -> Text {
//         Text {
//             raw: iced::canvas::Text {
//                 content,
//                 ..Default::default()
//             },
//         }
//     }
// }

// impl From<&str> for Text {
//     fn from(content: &str) -> Text {
//         String::from(content).into()
//     }
// }
