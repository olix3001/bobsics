use bobsics_render::BobsicsRenderer;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use self::widgets::{UniversalBrush, Vector2, BBox};

mod common;
mod label;
mod layout;

pub mod widgets {
    pub use super::common::*;
    pub use super::label::*;
    pub use super::layout::*;
}

pub struct BobsicsGUIApp {
    pub renderer: BobsicsRenderer,
    pub window: Window,
    pub event_loop: Option<EventLoop<()>>,
    pub widget: Option<Box<dyn Widget>>,
    pub brush: UniversalBrush,

    pub default_screen_size: (u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub struct Globals {
    pub screen_size: (u32, u32),
}

pub trait Widget {
    fn draw(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox; // x, y, width, height
    fn measure(&self, offset: Vector2, scale: Vector2, brush: &mut UniversalBrush) -> BBox;
    fn hover(&self);
    fn click(&self);
}

impl BobsicsGUIApp {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(1200, 700))
            .build(&event_loop)
            .unwrap();
        let renderer = pollster::block_on(BobsicsRenderer::new(&window));

        let brush = UniversalBrush::new(
            &renderer.device,
            renderer.format,
            &renderer.globals_bind_group_layout,
        );
        Self {
            renderer,
            window,
            event_loop: Some(event_loop),
            widget: None,
            brush,
            default_screen_size: (1200, 700),
        }
    }

    pub fn run(mut self) -> ! {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        self.renderer.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.renderer.resize(**new_inner_size);
                    }
                    _ => {}
                },

                Event::RedrawRequested(_) => {
                    let scale_factor = Vector2::new(
                        self.window.inner_size().width as f32 / self.default_screen_size.0 as f32,
                        self.window.inner_size().height as f32 / self.default_screen_size.1 as f32,
                    );
                    self.draw_widget(
                        &Globals {
                            screen_size: self.window.inner_size().into(),
                        },
                        scale_factor,
                    );
                    match self.renderer.render(&mut self.brush) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Outdated) => {}
                        Err(e) => eprintln!("{e:?}"),
                    }
                }

                _ => {}
            })
    }

    pub fn set_widget(&mut self, widget: Box<dyn Widget>) {
        self.widget = Some(widget);
    }

    pub fn draw_widget(&mut self, globals: &Globals, scale_factor: Vector2) {
        if self.widget.is_none() {
            println!("No widget set");
            return;
        }
        let widget = self.widget.as_ref().unwrap();
        // Draw widgets below each other
        let (_x, _y, _width, _height) =
            widget.draw(Vector2::ZERO, scale_factor, &mut self.brush, globals).into();
    }
}
