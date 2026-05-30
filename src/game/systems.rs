use crate::game::components::*;
use crate::game::input::InputState;
use crate::game::definitions::*;
use crate::game::map::MapManager;
use crate::{TILE_SIZE, MAX_MAP_WIDTH, MAX_MAP_HEIGHT};

use hecs::{Entity, World};
use std::f32::consts::PI;

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub fn PlayerMovementSystem(world: &mut World, delta_time: f32, map_walls: &[u16], input: &InputState) {
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

fn is_wall(check_x: f32, check_y: f32, map: &[u16]) -> bool {
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

#[allow(non_snake_case)]
pub fn AnimatorSystem(world: &mut World, delta_time: f32, map: &mut MapManager) {
    for (pos, animator, sprite) in world.query_mut::<(&Position, &mut SpriteAnimator,&mut Sprite)>() {
        if animator.playback_state != PlaybackState::Playing {
            continue;
        }

        if let Some(animation) = animator.animations.get(&animator.current_animation) {
            animator.timer += delta_time;
            if animator.timer >= animation.frame_duration {
                animator.timer -= animation.frame_duration;

                let next_frame = animator.current_frame + 1;

                if next_frame >= animation.frames_front.len() {
                    animator.current_frame = 0;
                    if animation.playback_mode != PlaybackMode::Loop {
                        animator.playback_state = PlaybackState::Stopped;
                    }
                }
                else {
                    animator.current_frame = next_frame;
                }
            }
            if animator.playback_state == PlaybackState::Playing {
                sprite.atlas_index_front = animation.frames_front[animator.current_frame] as u32;
                sprite.atlas_index_back = animation.frames_back[animator.current_frame] as u32;
            }
        }
    }

    for (pos, animator) in world.query_mut::<(&Position, &mut TextureAnimator)>() {
        if animator.playback_state != PlaybackState::Playing {
            continue;
        }
        if let Some(animation) = animator.animations.get(&animator.current_animation) {
            animator.timer += delta_time;
            if animator.timer >= animation.frame_duration {
                animator.timer -= animation.frame_duration;

                let next_frame = animator.current_frame + 1;
                
                if next_frame >= animation.frames.len() {
                    animator.current_frame = 0;
                    if animation.playback_mode != PlaybackMode::Loop {
                        animator.playback_state = PlaybackState::Stopped;
                    }
                }
                else {
                    animator.current_frame = next_frame;
                }
            }
            if animator.playback_state == PlaybackState::Playing {
                map.set_wall(pos.x as u32, pos.y as u32, animation.frames[animator.current_frame] as u16);
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn InteractSystem(world: &mut World, input: &mut InputState, player: &mut Option<Entity>, map_walls: &[u16]) {
    if !input.interact || player.is_none() {
        return;
    }
    input.interact = false;

    let player_entity = player.unwrap();
    
    let (player_x, player_y) = {
        let mut q = world.query_one::<&Position>(player_entity);
        let p = q.get().unwrap();
        (p.x, p.y)
    };

    let (dir_x, dir_y) = {
        let mut q = world.query_one::<&Rotation>(player_entity);
        let r = q.get().unwrap();
        (r.angle.cos(), r.angle.sin())
    };

    let mut map_x = player_x.floor() as i32;
    let mut map_y = player_y.floor() as i32;

    let delta_dist_x = (1.0 / dir_x).abs();
    let delta_dist_y = (1.0 / dir_y).abs();

    let step_x: i32;
    let step_y: i32;
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    if dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (player_x - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = ((map_x + 1) as f32 - player_x) * delta_dist_x;
    }

    if dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (player_y - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = ((map_y + 1) as f32 - player_y) * delta_dist_y;
    }

    let mut hit_distance: f32 = 0.0;
    let mut entity_hit: Option<Entity> = None;

    while hit_distance <= INTERACT_DISTANCE {
        if side_dist_x < side_dist_y {
            hit_distance = side_dist_x;
            side_dist_x += delta_dist_x;
            map_x += step_x;
        } else {
            hit_distance = side_dist_y;
            side_dist_y += delta_dist_y;
            map_y += step_y;
        }

        if hit_distance > INTERACT_DISTANCE {
            break;
        }

        if map_x >= 0 && map_x < MAX_MAP_WIDTH as i32 && map_y >= 0 && map_y < MAX_MAP_HEIGHT as i32 {
            let map_index = (map_y * MAX_MAP_WIDTH as i32 + map_x) as usize;
            let tile = map_walls[map_index];
            
            if tile != 0 {
                //println!("Hit wall at ({}, {})", map_x, map_y);
                entity_hit = find_interactable_at_position(world, map_x as f32, map_y as f32);
                break;
            }
        }
    }

    if let Some(entity) = entity_hit {
        let on_interact = {
            let mut q = world.query_one::<&Interactable>(entity);
            q.get().unwrap().on_interact
        };
        on_interact(world, player, entity);
    }
}

fn find_interactable_at_position(world: &World, x: f32, y: f32) -> Option<Entity> {
    for (entity, pos, _inte) in world.query::<(Entity, &Position, &Interactable)>().iter() {
        if pos.x == x && pos.y == y {
            return Some(entity);
        }
    }
    None
}

#[allow(non_snake_case)]
pub fn VentSystem(world: &mut World, delta_time: f32) {
    for (vent, texture_animator) in world.query_mut::<(&mut Vent, &mut TextureAnimator)>() {
        if vent.is_open { continue; }
        vent.timer += delta_time;
        //println!("Vent timer: {}", vent.timer);
        if vent.timer >= vent.time_to_open {
            vent.is_open = true;
            vent.timer = 0.0;
            texture_animator.current_animation = TextureAnimKey::Vent(VentAnim::Opening);
            texture_animator.playback_state = PlaybackState::Playing;
        }
    }
}