use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Window, WindowId};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, NamedKey};

const TITLE: &str = "Voidseek";

pub struct WindowApp {
    window: Option<Window>,
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes().with_title(TITLE)).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { 
                event: KeyEvent { 
                    logical_key: Key::Named(key), 
                    state: ElementState::Pressed, 
                    repeat, 
                    .. 
                }, 
                ..
            } => {
                match key {
                    NamedKey::Escape => {
                        event_loop.exit();
                    }
                    _ => (),
                }
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl WindowApp {
    pub fn new() -> Self {
        Self { window: None }
    }
}