use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Fullscreen, Window, WindowId};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{PhysicalKey, KeyCode};
use winit::dpi::LogicalSize;

const TITLE: &str = "Voidseek";

pub struct WindowApp {
    window: Option<Window>,
    fullscreen: bool,
    screen_width: f64,
    screen_height: f64,
}

impl WindowApp {
    fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
        if let Some(window) = &self.window {
            let fullscreen_mode = if self.fullscreen {
                Some(Fullscreen::Borderless(None))
            } else {
                None
            };
            window.set_fullscreen(fullscreen_mode);
        }
    }

    fn change_resolution(&mut self, width: f64, height: f64) {
        if width <= 0.0 || height <= 0.0 { return; }
        self.screen_width = width;
        self.screen_height = height;
        if let Some(window) = &self.window {
            let _ = window.request_inner_size(LogicalSize::new(self.screen_width, self.screen_height));
        }
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title(TITLE)
            .with_inner_size(LogicalSize::new(self.screen_width, self.screen_height))
            .with_resizable(false)
            .with_fullscreen(if self.fullscreen { Some(Fullscreen::Borderless(None)) } else { None });
        self.window = Some(event_loop.create_window(attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                //The close button was pressed; stopping
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { 
                event: KeyEvent { 
                    physical_key,
                    state: ElementState::Pressed, 
                    .. 
                }, 
                ..
            } => {
                match physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        event_loop.exit();
                    }
                    PhysicalKey::Code(KeyCode::KeyP) => {
                        self.toggle_fullscreen();
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
    pub fn new(screen_width: f64, screen_height: f64) -> Self {
        Self { window: None, fullscreen: false, screen_width: screen_width, screen_height: screen_height }
    }
}