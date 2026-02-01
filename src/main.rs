use nalgebra::Vector2;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;
use winit::{self, event};
struct Coord {
    x: i32,
    y: i32,
}

struct Direction {
    x: i32,
    y: i32,
}
struct Ray {
    origin: Vec<Coord>,
    direction: Vec<Direction>,
}

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes().with_title("Ray"))
                .unwrap(),
        )
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close was requested");
            }
            WindowEvent::RedrawRequested => {
                println!("clicked");
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                if let PhysicalKey::Code(KeyCode::ArrowRight) = event.physical_key {
                    if event.state.is_pressed() {
                        self.window.as_ref().unwrap().request_redraw();
                    }
                }
            }
            _ => (),
        }
    }
}
fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
