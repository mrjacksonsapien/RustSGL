mod sgl;

use std::{num::NonZeroU32, rc::Rc};

use rand::random;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use softbuffer::{Context, Surface};

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    context: Option<Context<Rc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        window.set_title("Lol");

        self.window = Some(window.clone());

        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.context = Some(context);
        self.surface = Some(surface);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(surface), Some(window)) = (&mut self.surface, &self.window) {
                    let (width, height) = {
                        let size = window.inner_size();
                        (
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        )
                    };

                    let mut buffer = surface.buffer_mut().unwrap();

                    for index in 0..(width.get() * height.get()) {
                        let color: u32 = random();
                        buffer[index as usize] = color & 0xFFFFFF;
                    }

                    buffer.present().unwrap();
                }
            }
            WindowEvent::Resized(size) => {
                if let (Some(surface), Some(_)) = (&mut self.surface, &self.window) {
                    let width = NonZeroU32::new(size.width).unwrap();
                    let height = NonZeroU32::new(size.height).unwrap();
                    surface.resize(width, height).unwrap();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
