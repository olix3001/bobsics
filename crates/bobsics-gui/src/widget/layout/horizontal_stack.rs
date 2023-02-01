use bobsics_render::Color;

use crate::{
    widgets::{UniversalBrush, Vector2, BBox},
    Globals, Widget,
};

use super::LayoutCommonOptions;

pub struct HorizontalStack {
    pub children: Vec<Box<dyn Widget>>,
    pub options: LayoutCommonOptions,
}

impl HorizontalStack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            options: LayoutCommonOptions {
                padding: Vector2::ZERO,
                margin: Vector2::ZERO,
                spacing: Vector2::ZERO,
            },
        }
    }

    pub fn with_padding(mut self, padding: Vector2) -> Self {
        self.options.padding = padding;
        self
    }

    pub fn with_margin(mut self, margin: Vector2) -> Self {
        self.options.margin = margin;
        self
    }

    pub fn with_spacing(mut self, spacing: Vector2) -> Self {
        self.options.spacing = spacing;
        self
    }

    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_children(mut self, children: Vec<Box<dyn Widget>>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn build(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Widget for HorizontalStack {
    fn draw(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox {
        let mut n_offset = offset + self.options.margin + self.options.padding;
        let mut max_width = 0.0;
        let mut max_height = 0.0;

        for child in &self.children {
            let (_, _, width, height) = child.draw(n_offset, scale, brush, globals).into();

            n_offset.x += width + self.options.spacing.x + self.options.padding.x;
            max_width += width + self.options.spacing.x;
            max_height = f32::max(max_height, height);
        }

        let bbox: BBox = (
            offset.x,
            offset.y,
            max_width + self.options.margin.x + 2.0 * self.options.padding.x,
            max_height + self.options.margin.y + 2.0 * self.options.padding.y,
        ).into();
        
        bbox.draw(Vector2::ZERO, brush, Color::from_hex(0x0000ff));
        bbox
    }

    fn measure(&self, offset: Vector2, scale: Vector2, brush: &mut UniversalBrush) -> BBox {
        let mut n_offset = offset + self.options.margin + self.options.padding;
        let mut max_width = 0.0;
        let mut max_height = 0.0;

        for child in &self.children {
            let (_, _, width, height) = child.measure(n_offset, scale, brush).into();

            n_offset.x += width + self.options.spacing.x;
            max_width += width + self.options.spacing.x;
            max_height = f32::max(max_height, height);
        }

        (
            offset.x,
            offset.y,
            max_width + self.options.margin.x + 2.0 * self.options.padding.x,
            max_height + self.options.margin.y + 2.0 * self.options.padding.y,
        ).into()
    }

    fn hover(&self) {
        todo!()
    }

    fn click(&self) {
        todo!()
    }
}

impl Default for HorizontalStack {
    fn default() -> Self {
        Self::new()
    }
}
