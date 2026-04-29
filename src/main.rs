use winit::event_loop::{ControlFlow, EventLoop};
use game::state::GameState;

pub mod game;
mod core;
use core::window_app::WindowApp;

const WINDOW_WIDTH: f64 = 480.0;
const WINDOW_HEIGHT: f64 = 270.0;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut game_state = GameState::new();
    game_state.start();

    let mut app = WindowApp::new(WINDOW_WIDTH, WINDOW_HEIGHT, game_state);
    event_loop.run_app(&mut app);
}