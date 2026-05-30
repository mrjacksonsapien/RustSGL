use crate::sgl::types::Scene;

use std::{num::NonZeroU32, rc::Rc};

use rand::random;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use softbuffer::{Context, Surface};

pub trait SGLApp {
    fn update(&mut self, scene: &mut Scene) -> Option<Scene>;
    fn on_input(&mut self, event: InputEvent, scene: &mut Scene);
}

struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, buffer: &mut [u32], width: u32, height: u32, scene: &Scene) {
        // TODO: Implement actual software rendering logic

        for index in 0..(width * height) {
            let color: u32 = random();
            buffer[index as usize] = color & 0xFFFFFF;
        }
    }
}

struct AppState<A: SGLApp> {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    context: Option<Context<Rc<Window>>>,
    renderer: Renderer,
    current_scene: Scene,
    user_app: A,
    title: String,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    // Keyboard inputs
    Keyboard {
        key: winit::keyboard::KeyCode,
        pressed: bool,
    },

    // Mouse
    MouseMotion {
        dx: f64,
        dy: f64,
    },
    MouseButton {
        button: MouseButton,
        pressed: bool,
    },

    // Controller
    ControllerButton {
        id: u32,
        button_index: u8,
        pressed: bool,
    },
    ControllerAxis {
        id: u32,
        axis_index: u8,
        value: f32,
    },
}

impl<A: SGLApp> ApplicationHandler for AppState<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        window.set_title(&self.title);

        self.window = Some(window.clone());

        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.context = Some(context);
        self.surface = Some(surface);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(new_scene) = self.user_app.update(&mut self.current_scene) {
            self.current_scene = new_scene;
        }

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
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = state == winit::event::ElementState::Pressed;

                self.user_app.on_input(
                    InputEvent::Keyboard { key, pressed },
                    &mut self.current_scene,
                );
            }
            WindowEvent::CursorMoved { position, .. } => {
                // TODO: Calculate previous x and y for actual dx and dy
                self.user_app.on_input(
                    InputEvent::MouseMotion {
                        dx: position.x,
                        dy: position.y,
                    },
                    &mut self.current_scene,
                );
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == winit::event::ElementState::Pressed;
                let my_button = match button {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    _ => return, // Ignore other mouse buttons for now
                };

                self.user_app.on_input(
                    InputEvent::MouseButton {
                        button: my_button,
                        pressed,
                    },
                    &mut self.current_scene,
                );
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

                    self.renderer.render(
                        &mut buffer,
                        width.get(),
                        height.get(),
                        &self.current_scene,
                    );

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

pub struct SGLContext<A: SGLApp> {
    event_loop: EventLoop<()>,
    state: AppState<A>,
}

impl<A: SGLApp> SGLContext<A> {
    pub fn new(scene: Scene, user_app: A, title: impl Into<String>) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let renderer = Renderer::new();

        event_loop.set_control_flow(ControlFlow::Poll);

        let state = AppState {
            window: None,
            surface: None,
            context: None,
            renderer,
            current_scene: scene,
            user_app,
            title: title.into(),
        };

        Self { event_loop, state }
    }

    pub fn run(self) {
        let mut state = self.state;
        self.event_loop.run_app(&mut state).unwrap();
    }
}
