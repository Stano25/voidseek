use crate::game::input::InputState;
use crate::{TILE_SIZE, MAX_MAP_WIDTH, MAX_MAP_HEIGHT};
use std::f32::consts::PI;

const PLAYER_RADIUS: f32 = 10.0;

enum Axis {
    X,
    Y
}

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

            if !self.is_wall(move_x, Axis::X, map) {
                self.x += move_x;
            }
            if !self.is_wall(move_y, Axis::Y, map) {
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

    fn is_wall(&self, velocity: f32, axis: Axis, map: &[u32]) -> bool {
        let inverted_size = 1.0 / TILE_SIZE as f32;

        let player_rad = PLAYER_RADIUS * inverted_size;

        let old_x = self.x * inverted_size;
        let old_y = self.y * inverted_size;

        let check_sphere = |x_pos: f32, y_pos: f32| -> bool {
            let min_x = (x_pos - player_rad).floor() as i32;
            let max_x = (x_pos + player_rad).floor() as i32;
            let min_y = (y_pos - player_rad).floor() as i32;
            let max_y = (y_pos + player_rad).floor() as i32;

            for x in min_x..=max_x {
                for y in min_y..=max_y{
                    if x < 0 || x >= MAX_MAP_WIDTH as i32 || y < 0 || y >= MAX_MAP_HEIGHT as i32 {
                        return true; // Ak sa hráč pokúša ísť mimo mapy
                    }

                    let map_index = (y as usize) * MAX_MAP_WIDTH as usize + (x as usize);
                    if let Some(&tile) = map.get(map_index) {
                        if tile != 0 {
                            return true;
                        }
                    }
                }
            }

            false
        };

        if velocity != 0.0 {
            match axis {
                Axis::X => {
                    let new_x = (self.x + velocity) * inverted_size;
                    check_sphere(new_x, old_y)
                }
                Axis::Y => {
                    let new_y = (self.y + velocity) * inverted_size;
                    check_sphere(old_x, new_y)
                }
            }
        } else {
            false
        }
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}