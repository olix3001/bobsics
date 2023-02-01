use bobsics_render::BobsicsRenderer;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use self::widgets::{BBox, GUIEvent, UniversalBrush, Vector2};

mod button;
mod common;
mod label;
mod layout;

pub mod widgets {
    pub use super::button::*;
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

    pub mouse_pos: Vector2,
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
    fn measure(
        &self,
        offset: Vector2,
        scale: Vector2,
        brush: &mut UniversalBrush,
        globals: &Globals,
    ) -> BBox;
    fn handle_event(
        &mut self,
        window: &Window,
        brush: &mut UniversalBrush,
        offset: Vector2,
        scale: Vector2,
        event: &GUIEvent,
        globals: &Globals,
    );
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
            mouse_pos: Vector2::ZERO,
            widget: None,
            brush,
            default_screen_size: (1200, 700),
        }
    }

    pub fn run(mut self) -> ! {
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                let scale_factor = Vector2::new(
                    self.window.inner_size().width as f32 / self.default_screen_size.0 as f32,
                    self.window.inner_size().height as f32 / self.default_screen_size.1 as f32,
                );
                match event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == self.window.id() => match event {
                        WindowEvent::CursorMoved {
                            position,
                            ..
                        } => {
                            let scale_factor = Vector2::new(
                                self.window.inner_size().width as f32
                                    / self.default_screen_size.0 as f32,
                                self.window.inner_size().height as f32
                                    / self.default_screen_size.1 as f32,
                            );
                            let mouse_position: Vector2 =
                                (position.x as f32, position.y as f32).into();
                            self.mouse_pos = mouse_position;
                            if let Some(widget) = &mut self.widget {
                                // TODO: make globals global
                                widget.handle_event(
                                    &self.window,
                                    &mut self.brush,
                                    Vector2::ZERO,
                                    scale_factor,
                                    &GUIEvent::CursorMoved(mouse_position),
                                    &Globals {
                                        screen_size: self.window.inner_size().into(),
                                    },
                                );
                            }
                        }
                        WindowEvent::MouseInput {
                            state,
                            ..
                        } => {
                            let scale_factor = Vector2::new(
                                self.window.inner_size().width as f32
                                    / self.default_screen_size.0 as f32,
                                self.window.inner_size().height as f32
                                    / self.default_screen_size.1 as f32,
                            );
                            if let Some(widget) = &mut self.widget {
                                match state {
                                    winit::event::ElementState::Pressed => {
                                        widget.handle_event(
                                            &self.window,
                                            &mut self.brush,
                                            Vector2::ZERO,
                                            scale_factor,
                                            &GUIEvent::MousePressed(self.mouse_pos),
                                            &Globals {
                                                screen_size: self.window.inner_size().into(),
                                            },
                                        );
                                    }
                                    winit::event::ElementState::Released => {
                                        widget.handle_event(
                                            &self.window,
                                            &mut self.brush,
                                            Vector2::ZERO,
                                            scale_factor,
                                            &GUIEvent::MouseReleased(self.mouse_pos),
                                            &Globals {
                                                screen_size: self.window.inner_size().into(),
                                            },
                                        );
                                    }
                                }
                            }
                        }
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
                        println!("Redraw");
                        self.draw_widget(
                            &Globals {
                                screen_size: self.window.inner_size().into(),
                            },
                            scale_factor,
                        );
                        match self.renderer.render(&mut self.brush) {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            Err(wgpu::SurfaceError::Outdated) => {}
                            Err(e) => eprintln!("{e:?}"),
                        }
                    }

                    _ => {}
                }
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
        let (_x, _y, _width, _height) = widget
            .draw(Vector2::ZERO, scale_factor, &mut self.brush, globals)
            .into();
    }
}
