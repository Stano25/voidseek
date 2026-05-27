use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop};
use winit::window::{Fullscreen, Window, WindowId};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{PhysicalKey, KeyCode};
use winit::dpi::LogicalSize;
use std::time::Instant;
use crate::core::renderer::{self, WgpuState, SpriteInstance};
use crate::game::state::GameState;

const TITLE: &str = "Voidseek";

pub struct WindowApp {
    window: Option<Arc<Window>>,
    renderer: Option<WgpuState>,
    game: GameState,
    last_time: Instant,
    fullscreen: bool,
    mouse_locked: bool,
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

    pub fn toggle_mouse_lock(&mut self) {
        self.mouse_locked = !self.mouse_locked;
        if let Some(window) = &self.window {
            if self.mouse_locked {
                let _ = window.set_cursor_grab(winit::window::CursorGrabMode::Locked)
                    .or_else(|_| window.set_cursor_grab(winit::window::CursorGrabMode::Confined));
                window.set_cursor_visible(false);
            } else {
                let _ = window.set_cursor_grab(winit::window::CursorGrabMode::None);
                window.set_cursor_visible(true);
            }
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
    
        
        if let Some(renderer) = &mut self.renderer {
            renderer.update_map(&self.game.get_map_data());
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { 
                event: KeyEvent { 
                    physical_key,
                    state, 
                    .. 
                }, 
                ..
            } => {
                match (physical_key, state) {
                    (PhysicalKey::Code(KeyCode::Escape), ElementState::Pressed) => {
                        event_loop.exit();
                    }
                    (PhysicalKey::Code(KeyCode::KeyP), ElementState::Pressed) => {
                        self.toggle_fullscreen();
                    }
                    (PhysicalKey::Code(KeyCode::KeyL), ElementState::Pressed) => {
                        self.toggle_mouse_lock();
                    }

                    (PhysicalKey::Code(KeyCode::KeyW), state) => {
                        self.game.input_state.forward = state == ElementState::Pressed;
                    }
                    (PhysicalKey::Code(KeyCode::KeyS), state) => {
                        self.game.input_state.backward = state == ElementState::Pressed;
                    }
                    (PhysicalKey::Code(KeyCode::KeyA), state) => {
                        self.game.input_state.left = state == ElementState::Pressed;
                    }
                    (PhysicalKey::Code(KeyCode::KeyD), state) => {
                        self.game.input_state.right = state == ElementState::Pressed;
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
                let dt = self.last_time.elapsed().as_secs_f32();
                self.last_time = Instant::now();
                
                self.game.update(dt);

                if let Some(renderer) = &mut self.renderer {
                    if let Some((cam_x, cam_y, cam_angle)) = self.game.get_camera_info() {
                        renderer.update_camera(cam_x, cam_y, cam_angle);
                    }

                    let sprite_instances = self.game.get_sprites();

                    renderer.update_sprites(&sprite_instances);

                    renderer.render();
                }
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: winit::event::DeviceId, event: winit::event::DeviceEvent) {
        if let winit::event::DeviceEvent::MouseMotion { delta: (dx, _dy) } = event {
            if self.mouse_locked {
                self.game.input_state.mouse_dx += dx;
            }
        }
    }
}

impl WindowApp {
    pub fn new(screen_width: f64, screen_height: f64, game: GameState) -> Self {
        Self { 
            window: None, 
            renderer: None,
            game,
            last_time: Instant::now(),
            fullscreen: false, 
            mouse_locked: false,
            screen_width, 
            screen_height 
        }
    }
}