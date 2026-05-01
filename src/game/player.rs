use crate::game::input::InputState;
use std::f32::consts::PI;

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
            speed: 25000.0,
            sensitivity: 0.0015,
        }
    }

    pub fn start(&mut self) {
        self.calculate_delta();
    }

    pub fn update(&mut self, delta_time: f32, input: &mut InputState) {
        self.handle_rotation(input);
        self.handle_input(delta_time, input);

        println!("Player position: ({:.2}, {:.2}), angle: {:.2} radians", self.x, self.y, self.angle);
    }

    fn handle_input(&mut self,delta_time: f32 ,input: &InputState) {
        if input.forward {
            self.x += self.delta_x * delta_time * self.speed;
            self.y += self.delta_y * delta_time * self.speed;
        }
        if input.backward {
            self.x -= self.delta_x * delta_time * self.speed;
            self.y -= self.delta_y * delta_time * self.speed;
        }

        let strafe_x = (self.angle + PI / 2.0).cos() * self.sensitivity as f32;
        let strafe_y = (self.angle + PI / 2.0).sin() * self.sensitivity as f32;

        if input.right {
            self.x += strafe_x * delta_time * self.speed;
            self.y += strafe_y * delta_time * self.speed;
        }
        if input.left {
            self.x -= strafe_x * delta_time * self.speed;
            self.y -= strafe_y * delta_time * self.speed;
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
        self.delta_x = self.angle.cos() * self.sensitivity as f32;
        self.delta_y = self.angle.sin() * self.sensitivity as f32;
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }
}