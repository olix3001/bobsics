use crate::{
    widgets::{UniversalBrush, Vector2},
    Globals, Widget,
};

use super::LayoutCommonOptions;

pub struct VerticalStack {
    pub children: Vec<Box<dyn Widget>>,
    pub options: LayoutCommonOptions,
}

impl VerticalStack {
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

impl Widget for VerticalStack {
    fn draw(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> (f32, f32, f32, f32) {
        let mut offset = offset + self.options.margin * scale;
        let mut max_width = 0.0;
        let mut max_height = 0.0;

        for child in &self.children {
            let (_, _, width, height) = child.draw(offset, scale, brush, globals);

            offset.y += height + self.options.spacing.y * scale.y;
            max_width = f32::max(max_width, width);
            max_height += height + self.options.spacing.y * scale.y;
        }

        (
            offset.x,
            offset.y,
            max_width + self.options.margin.x * scale.x,
            max_height + self.options.margin.y * scale.y,
        )
    }

    fn hover(&self) {
        todo!()
    }

    fn click(&self) {
        todo!()
    }
}

impl Default for VerticalStack {
    fn default() -> Self {
        Self::new()
    }
}
