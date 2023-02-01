use bobsics_render::Color;

use crate::{
    widgets::{BBox, Font, GUIEvent, Label, UniversalBrush, Vector2},
    Globals, Widget,
};

pub type ButtonClickCallback = Box<dyn Fn(ButtonClickEvent)->bool>;

pub struct ButtonClickEvent<'a> {
    pub button: &'a mut Button,
    pub mouse_position: Vector2,
    pub mouse_position_relative: Vector2,
}

pub struct Button {
    pub text: String,
    pub options: ButtonOptions,
    pub on_click: Option<ButtonClickCallback>,

    // private
    _label: Label,
    _is_hovered: bool,
}

pub struct ButtonOptions {
    pub color: Color,
    pub hover_color: Color,
    pub text_color: Color,
    pub font: Font,
    pub scale: f32,
    pub padding: Vector2,
    pub margin: Vector2,
    pub radius: f32,

    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl Default for ButtonOptions {
    fn default() -> Self {
        Self {
            color: Color::from_hex(0x1a6bf5),
            hover_color: Color::from_hex(0x1a6bf5).with_alpha(0.5),
            text_color: Color::WHITE,
            font: Font::default(),
            scale: 16.0,
            padding: Vector2::new(8.0, 5.0),
            margin: Vector2::new(8.0, 5.0),
            radius: 4.0,
            width: None,
            height: None,
        }
    }
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            options: ButtonOptions::default(),
            on_click: None,
            _label: Label::new(text, 16.0),
            _is_hovered: false,
        }
    }

    pub fn with_options(mut self, options: ButtonOptions) -> Self {
        self.options = options;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.options.color = color;
        self
    }

    pub fn with_hover_color(mut self, color: Color) -> Self {
        self.options.hover_color = color;
        self
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.options.text_color = color;
        self
    }

    pub fn with_font(mut self, font: Font) -> Self {
        self.options.font = font;
        self
    }

    pub fn with_size(mut self, scale: f32) -> Self {
        self.options.scale = scale;
        self._label = Label::new(&self.text, scale);
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self._label = Label::new(text, self.options.scale);
        self
    }

    pub fn with_padding(mut self, padding: Vector2) -> Self {
        self.options.padding = padding;
        self
    }

    pub fn with_margin(mut self, margin: Vector2) -> Self {
        self.options.margin = margin;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.options.radius = radius;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.options.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.options.height = Some(height);
        self
    }

    pub fn on_click<F>(mut self, on_click: F) -> Self
    where
        F: Fn(ButtonClickEvent)->bool + 'static,
    {
        self.on_click = Some(Box::new(on_click));
        self
    }

    pub fn build(self) -> Box<Self> {
        Box::new(self)
    }

    // ====< Setters without ownership >====
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self._label = Label::new(text, self.options.scale);
    }

    // ====< Event handlers >====
    fn handle_mouse_move(
        &mut self,
        window: &winit::window::Window,
        brush: &mut UniversalBrush,
        offset: Vector2,
        scale: Vector2,
        mouse_position: Vector2,
        globals: &Globals,
    ) {
        // Set hovered state
        let ns = self
            .measure_hitbox(offset, scale, brush, globals)
            .contains(mouse_position);
        if self._is_hovered != ns {
            self._is_hovered = ns;
            window.set_cursor_icon(if ns {
                winit::window::CursorIcon::Hand
            } else {
                winit::window::CursorIcon::Arrow
            });
            window.request_redraw();
        }
    }
    fn handle_mouse_click(
        &mut self,
        window: &winit::window::Window,
        brush: &mut UniversalBrush,
        offset: Vector2,
        scale: Vector2,
        mouse_position: Vector2,
        globals: &Globals,
    ) {
        if self.on_click.is_none() {
            return;
        }
        // Check if the button was clicked
        if self
            .measure_hitbox(offset, scale, brush, globals)
            .contains(mouse_position)
        {
            // Temporary solution
            let function = self.on_click.take().unwrap();
            let event = ButtonClickEvent {
                button: self,
                mouse_position,
                mouse_position_relative: mouse_position - offset,
            };
            if function(event) { window.request_redraw(); }
            self.on_click = Some(function);
        }
    }

    fn measure_hitbox(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox {
        let offset = offset + self.options.margin * scale;

        // Get label size
        let label_bbox = self._label.measure(offset, scale, brush, globals);

        // Get button size
        let button_size = Vector2::new(
            self.options
                .width
                .unwrap_or(label_bbox.width() + self.options.padding.x * 2.0) * scale.x,
            self.options
                .height
                .unwrap_or(label_bbox.height() + self.options.padding.y * 2.0) * scale.y,
        );
        
        BBox::new(offset, offset + button_size)
    }
}

impl Widget for Button {
    fn draw(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox {
        let n_offset = offset + self.options.margin * scale;

        // Get label size
        let label_bbox = self._label.measure(n_offset, scale, brush, globals);

        // Get button size
        let button_size = Vector2::new(
            self.options
                .width
                .unwrap_or(label_bbox.width() + self.options.padding.x * 2.0) * scale.x,
            self.options
                .height
                .unwrap_or(label_bbox.height() + self.options.padding.y * 2.0) * scale.y,
        );

        // Get text offset
        let text_offset = Vector2::new(
            (button_size.x - label_bbox.width()) / 2.0,
            (button_size.y - label_bbox.height()) / 2.0,
        );

        // Draw background
        brush.queue_quad_raw(bobsics_render::Quad {
            top_left: n_offset.into(),
            bottom_right: (n_offset + button_size).into(),
            color: if self._is_hovered {
                self.options.hover_color.into()
            } else {
                self.options.color.into()
            },
            border_radius: self.options.radius,
            border_color: Color::TRANSPARENT.into(),
            border_width: 0.0,
        }).expect("Failed to draw button background");

        // Draw text
        self._label
            .draw(n_offset + text_offset, scale, brush, globals);

        self.measure(offset, scale, brush, globals)
    }

    fn measure(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox {
        let offset = offset;

        // Get label size
        let label_bbox = self._label.measure(offset, scale, brush, globals);

        // Get button size
        let button_size = Vector2::new(
            self.options
                .width
                .unwrap_or(label_bbox.width() + self.options.padding.x * 2.0) * scale.x + self.options.margin.x * 2.0 * scale.x,
            self.options
                .height
                .unwrap_or(label_bbox.height() + self.options.padding.y * 2.0) * scale.y + self.options.margin.y * 2.0 * scale.y,
        );
        
        BBox::new(offset, offset + button_size)
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
        match event {
            GUIEvent::CursorMoved(mouse_position) => {
                self.handle_mouse_move(window, brush, offset, scale, *mouse_position, globals)
            }
            GUIEvent::MousePressed(mouse_position) => {
                self.handle_mouse_click(window, brush, offset, scale, *mouse_position, globals)
            }
            _ => {}
        }
    }
}
