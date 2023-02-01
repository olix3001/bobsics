use bobsics_render::Color;

use crate::{
    widgets::{BBox, GUIEvent, UniversalBrush, Vector2},
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
    ) -> BBox {
        let mut n_offset = offset + self.options.margin * scale + self.options.padding * scale;
        let mut max_width = 0.0;
        let mut max_height = 0.0;

        for child in &self.children {
            let (_, _, width, height) = child.draw(n_offset, scale, brush, globals).into();

            n_offset.y += height + self.options.spacing.y * scale.y;
            max_width = f32::max(max_width, width);
            max_height += height + self.options.spacing.y * scale.y;
        }

        let bbox: BBox = (
            offset.x,
            offset.y,
            max_width + scale.x * (self.options.margin.x + 2.0 * self.options.padding.x),
            max_height
                + scale.y
                    * (self.options.margin.y + 2.0 * self.options.padding.y
                        - self.options.spacing.y),
        )
            .into();

        bbox
    }

    fn measure(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox {
        let mut n_offset = offset + self.options.margin * scale + self.options.padding * scale;
        let mut max_width = 0.0;
        let mut max_height = 0.0;

        for child in &self.children {
            let (_, _, width, height) = child.measure(n_offset, scale, brush, globals).into();

            n_offset.y += height + self.options.spacing.y * scale.y;
            max_width = f32::max(max_width, width);
            max_height += height + self.options.spacing.y * scale.y;
        }

        (
            offset.x,
            offset.y,
            max_width + scale.x * (self.options.margin.x + 2.0 * self.options.padding.x),
            max_height
                + scale.y
                    * (self.options.margin.y + 2.0 * self.options.padding.y
                        - self.options.spacing.y),
        )
            .into()
    }

    fn handle_event(
        &mut self,
        window: &winit::window::Window,
        brush: &mut UniversalBrush,
        offset: Vector2,
        scale: Vector2,
        event: &GUIEvent,
        globals: &Globals,
    ) {
        // TODO: Ignore mouse events if the mouse is not over the widget

        // Figure out which child is hovered
        let mut n_offset = offset + self.options.margin * scale + self.options.padding * scale;

        for child in &mut self.children {
            let bbox = child.measure(n_offset, scale, brush, globals);
            child.handle_event(window, brush, n_offset, scale, event, globals);
            n_offset.y += bbox.height() + self.options.spacing.y * scale.y;
        }
    }
}

impl Default for VerticalStack {
    fn default() -> Self {
        Self::new()
    }
}
