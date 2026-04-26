use winit::event_loop::{ControlFlow, EventLoop};

mod core;
use core::window_app::WindowApp;

const WINDOW_WIDTH: f64 = 960.0;
const WINDOW_HEIGHT: f64 = 540.0;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = WindowApp::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    event_loop.run_app(&mut app);
}