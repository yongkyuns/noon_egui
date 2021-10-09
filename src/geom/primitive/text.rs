use iced::canvas::Frame;
use iced::canvas::Geometry;
use iced::Font;
use iced::Size;

use crate::Color;
use crate::Point;

#[derive(Debug, Default)]
pub struct Text {
    pub(crate) raw: iced::canvas::Text,
}

impl Text {
    pub fn at(mut self, position: Point) -> Self {
        self.raw.position = position.into();
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.raw.color = color;
        self
    }
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.raw.size = size;
        self
    }
    pub fn with_font(mut self, font: Font) -> Self {
        self.raw.font = font;
        self
    }
    pub fn scale_by(mut self, scale: f32) -> Self {
        self.raw.size *= scale;
        self
    }
    pub fn draw(self, size: Size) -> Geometry {
        let mut frame = Frame::new(size);
        frame.fill_text(self.raw);
        frame.into_geometry()
    }
    pub fn position(&self) -> Point {
        self.raw.position.into()
    }
}

impl Into<iced::canvas::Text> for Text {
    fn into(self) -> iced::canvas::Text {
        self.raw
    }
}

impl From<String> for Text {
    fn from(content: String) -> Text {
        Text {
            raw: iced::canvas::Text {
                content,
                ..Default::default()
            },
        }
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Text {
        String::from(content).into()
    }
}
