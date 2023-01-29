use bobsics_render::BobsicsRenderer;
use winit::{window::{WindowBuilder, Window}, dpi::PhysicalSize, event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent}};

fn main() {
    let bobsics = Bobsics::new();
    bobsics.run();
}

struct Bobsics {
    event_loop: EventLoop<()>,
    window: Window,
    renderer: BobsicsRenderer
}

impl Bobsics {
    pub fn new() -> Self {

        // Init event loop
        let event_loop = EventLoop::new();

        // Init window
        let window = WindowBuilder::new()
        .with_title("Bobsics")
        .with_inner_size(PhysicalSize::new(1200, 700))
        .build(&event_loop).unwrap();

        let renderer = pollster::block_on(BobsicsRenderer::new(&window));

        Self {
            event_loop,
            window,
            renderer
        }
    }

    pub fn run(mut self) -> ! {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id
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
                    match self.renderer.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Outdated) => { }
                        Err(e) => eprintln!("{e:?}"),
                    }
                }

                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size)
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
