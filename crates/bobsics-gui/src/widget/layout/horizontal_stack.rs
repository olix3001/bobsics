use crate::{
    widgets::{UniversalBrush, Vector2},
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
    ) -> (f32, f32, f32, f32) {
        let mut offset = offset + self.options.margin;
        let mut max_width = 0.0;
        let mut max_height = 0.0;

        for child in &self.children {
            let (_, _, width, height) = child.draw(offset, scale, brush, globals);

            offset.x += width + self.options.spacing.x;
            max_width += width + self.options.spacing.x;
            max_height = f32::max(max_height, height);
        }

        (
            offset.x,
            offset.y,
            max_width + self.options.margin.x,
            max_height + self.options.margin.y,
        )
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
