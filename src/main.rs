use winit::event_loop::{ControlFlow, EventLoop};

mod core;
use core::window_app::WindowApp;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = WindowApp::new();
    event_loop.run_app(&mut app);
}