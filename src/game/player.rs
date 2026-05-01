use crate::game::input::InputState;
use crate::{TILE_SIZE, MAX_MAP_WIDTH, MAX_MAP_HEIGHT};
use std::char::MAX;
use std::f32::consts::PI;

const PLAYER_RADIUS: f32 = 10.0;

pub struct Player {
    x: f32,
    y: f32,
    angle: f32,
    delta_x: f32,
    delta_y: f32,
    speed: f32,
    sensitivity: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 300.0,
            y: 300.0,
            angle: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            speed: 50.0,
            sensitivity: 0.0015,
        }
    }

    pub fn start(&mut self) {
        self.calculate_delta();
    }

    pub fn update(&mut self, delta_time: f32, input: &mut InputState, map: &[u32]) {
        self.handle_rotation(input);
        self.handle_input(delta_time, input, map);

        //println!("Player position: ({:.2}, {:.2}), angle: {:.2} radians", self.x, self.y, self.angle);
    }

    fn handle_input(&mut self,delta_time: f32 ,input: &InputState, map: &[u32]) {
        let mut move_x = 0.0;
        let mut move_y = 0.0;

        if input.forward {
            move_x += self.delta_x;
            move_y += self.delta_y;
        }

        if input.backward {
            move_x -= self.delta_x;
            move_y -= self.delta_y;
        }

        let strafe_x = (self.angle + PI / 2.0).cos();
        let strafe_y = (self.angle + PI / 2.0).sin();

        if input.right {
            move_x += strafe_x;
            move_y += strafe_y;
        }

        if input.left {
            move_x -= strafe_x;
            move_y -= strafe_y;
        }

        // Normalizacia pohybu
        let magnitude = (move_x * move_x + move_y * move_y).sqrt();
        
        if magnitude > 0.0 {
            move_x = (move_x / magnitude) * self.speed * delta_time;
            move_y = (move_y / magnitude) * self.speed * delta_time;

            let (x_colided, y_colided) = self.is_wall(move_x, move_y, map);

            if !x_colided {
                self.x += move_x;
            }
            if !y_colided {
                self.y += move_y;
            }
        }
    }

    fn handle_rotation(&mut self, input: &mut InputState) {
        if input.mouse_dx != 0.0 {
            self.angle += (input.mouse_dx * self.sensitivity) as f32;
            if self.angle < 0.0 {
                self.angle += 2.0 * PI;
            } else if self.angle >= 2.0 * PI {
                self.angle -= 2.0 * PI;
            }
            self.calculate_delta();

            input.mouse_dx = 0.0;
        }
    }

    fn calculate_delta(&mut self) {
        self.delta_x = self.angle.cos();
        self.delta_y = self.angle.sin();
    }

    fn is_wall(&self, velocity_x: f32, velocity_y: f32, map: &[u32]) -> (bool, bool) {
        let center_x = (self.x + velocity_x)/TILE_SIZE as f32;
        let center_y = (self.y + velocity_y)/TILE_SIZE as f32;

        let player_radius = PLAYER_RADIUS / TILE_SIZE as f32;

        let x_pos = self.x / TILE_SIZE as f32;
        let y_pos = self.y / TILE_SIZE as f32;

        let mut x_colided = false;
        let mut y_colided = false;

        for x in (center_x-player_radius).floor() as i32..(center_x+player_radius).ceil() as i32 {
            for y in (y_pos-player_radius).floor() as i32..(y_pos+player_radius).ceil() as i32 {
                if x < 0 || x >= (MAX_MAP_WIDTH * TILE_SIZE) as i32 || y < 0 || y >= (MAX_MAP_HEIGHT * TILE_SIZE) as i32 {
                    continue;
                }

                let map_x = x as usize;
                let map_y = y as usize;
                let map_index = map_y * MAX_MAP_WIDTH as usize + map_x;
                if let Some(&tile) = map.get(map_index) && tile != 0{
                    x_colided = true;
                }
            }
        }

        for x in (x_pos-player_radius).floor() as i32..(x_pos+player_radius).ceil() as i32 {
            for y in (center_y-player_radius).floor() as i32..(center_y+player_radius).ceil() as i32 {
                if x < 0 || x >= (MAX_MAP_WIDTH * TILE_SIZE) as i32 || y < 0 || y >= (MAX_MAP_HEIGHT * TILE_SIZE) as i32 {
                    continue;
                }

                let map_x = x as usize;
                let map_y = y as usize;
                let map_index = map_y * MAX_MAP_WIDTH as usize + map_x;
                if let Some(&tile) = map.get(map_index) && tile != 0{
                    y_colided = true;
                }
            }
        }
        
        (x_colided, y_colided)
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}