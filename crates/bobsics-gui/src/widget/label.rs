use bobsics_render::{Color, Quad};
use wgpu_glyph::Text;

use crate::widgets::{UniversalBrush, Vector2, BBox};

use super::{widgets::Font, Globals, Widget};

pub struct Label {
    pub text: String,
    pub color: Color,
    pub font: Font,
    pub scale: f32,
}

impl Label {
    // Create a new label builder
    pub fn new(text: &str, size: f32) -> Self {
        Self {
            text: text.to_string(),
            color: Color::from_hex(0xffffff),
            font: Font::default(),
            scale: size,
        }
    }

    // Build the label
    pub fn build(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn with_size(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }
}

impl Widget for Label {
    fn draw(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox {
        // Draw the text
        let section = wgpu_glyph::Section {
            screen_position: offset.into(),
            bounds: (globals.screen_size.0 as f32, globals.screen_size.1 as f32),
            text: vec![Text::new(&self.text)
                .with_color(self.color)
                .with_scale(self.scale * scale.x)],
            ..Default::default()
        };

        brush.queue_text_raw(&section).unwrap();

        // Measure the text and return the bounding box
        let bbox = self.measure(offset, scale, brush);
        bbox.draw(Vector2::ZERO, brush, Color::from_hex(0x00ff00));
        bbox
    }

    fn measure(&self, offset: Vector2, scale: Vector2, brush: &mut UniversalBrush) -> BBox {
        let section = wgpu_glyph::Section {
            screen_position: offset.into(),
            bounds: (1000.0, 1000.0),
            text: vec![Text::new(&self.text)
                .with_color(self.color)
                .with_scale(self.scale * scale.x)],
            ..Default::default()
        };

        let bbox = brush.measure(&section);
        (offset.x, offset.y, bbox.0, bbox.1).into()
    }

    fn hover(&self) {
        todo!()
    }

    fn click(&self) {
        todo!()
    }
}
