use crate::game::components::{Position, Rotation, Velocity, PlayerController};
use crate::game::input::InputState;
use crate::game::definitions::PLAYER_RADIUS;
use crate::{TILE_SIZE, MAX_MAP_WIDTH, MAX_MAP_HEIGHT};
use hecs::World;
use std::f32::consts::PI;

pub fn PlayerRotationSystem(world: &mut World, input: &mut InputState) {
    for (rot, vel, control) in world.query_mut::<(&mut Rotation, &mut Velocity, &PlayerController)>() {
        if input.mouse_dx != 0.0 {
            rot.angle += (input.mouse_dx as f32) * control.sensitivity;
            if rot.angle < 0.0 {
                rot.angle += 2.0 * PI;
            } else if rot.angle >= 2.0 * PI {
                rot.angle -= 2.0 * PI;
            }
            input.mouse_dx = 0.0;
        }

        // Smerové vektory vypočítané z uhla
        vel.dx = rot.angle.cos();
        vel.dy = rot.angle.sin();
    }
}

pub fn PlayerMovementSystem(world: &mut World, delta_time: f32, map_walls: &[u32], input: &InputState) {
    for (pos, rot, vel) in world.query_mut::<(&mut Position, &mut Rotation, &mut Velocity)>() {
        let mut move_x: f32 = 0.0;
        let mut move_y: f32 = 0.0;

        if input.forward {
            move_x += vel.dx;
            move_y += vel.dy;
        }

        if input.backward {
            move_x -= vel.dx;
            move_y -= vel.dy;
        }

        let strafe_x = (rot.angle + PI / 2.0).cos();
        let strafe_y = (rot.angle + PI / 2.0).sin();

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
            move_x = (move_x / magnitude) * vel.speed * delta_time;
            move_y = (move_y / magnitude) * vel.speed * delta_time;

            if !is_wall(pos.x + move_x, pos.y, map_walls) {
                pos.x += move_x;
            }
            if !is_wall(pos.x, pos.y + move_y, map_walls) {
                pos.y += move_y;
            }
        }
    }
}

fn is_wall(check_x: f32, check_y: f32, map: &[u32]) -> bool {
        let inverted_size = 1.0 / TILE_SIZE as f32;

        let player_rad = PLAYER_RADIUS * inverted_size;

        let min_x = (check_x - player_rad).floor() as i32;
        let max_x = (check_x + player_rad).floor() as i32;
        let min_y = (check_y - player_rad).floor() as i32;
        let max_y = (check_y + player_rad).floor() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y{
                if x < 0 || x >= MAX_MAP_WIDTH as i32 || y < 0 || y >= MAX_MAP_HEIGHT as i32 {
                    return true; // Ak sa hráč pokúša ísť mimo mapy
                }

                let map_index = (y as usize) * MAX_MAP_WIDTH as usize + (x as usize);
                if map.get(map_index).is_some_and(|&tile| tile != 0) {
                    return true;
                }
            }
        }

        false
    }

