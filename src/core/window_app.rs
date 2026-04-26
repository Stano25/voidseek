use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Fullscreen, Window, WindowId};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{PhysicalKey, KeyCode};
use winit::dpi::LogicalSize;
use crate::core::renderer::WgpuState;

const TITLE: &str = "Voidseek";

pub struct WindowApp {
    window: Option<Arc<Window>>,
    renderer: Option<WgpuState>,
    fullscreen: bool,
    screen_width: f64,
    screen_height: f64,
}

impl WindowApp {
    pub fn toggle_fullscreen(&mut self) {
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

    pub fn change_resolution(&mut self, width: f64, height: f64) {
        if width > 0.0 && height > 0.0 {
            self.screen_width = width;
            self.screen_height = height;
            if let Some(window) = &self.window {
                let _ = window.request_inner_size(LogicalSize::new(self.screen_width, self.screen_height));
            }
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
        
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        self.window = Some(window.clone());
        
        // Inicializácia WgpuState cez pollster (asynchrónna operácia v synchrónnom kontexte)
        let wgpu_state = pollster::block_on(WgpuState::new(window));
        self.renderer = Some(wgpu_state);
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
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size.width, new_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render();
                }
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

impl WindowApp {
    pub fn new(screen_width: f64, screen_height: f64) -> Self {
        Self { 
            window: None, 
            renderer: None,
            fullscreen: false, 
            screen_width, 
            screen_height 
        }
    }
}