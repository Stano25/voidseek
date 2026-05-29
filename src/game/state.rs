use crate::game::input::InputState;
use crate::core::renderer::{SpriteInstance};
use hecs::{Entity, World};
use crate::game::components::{Position, Rotation, Velocity, PlayerController, Sprite, Interactable, TextureAnimator};
use crate::game::systems::*;
use crate::game::definitions::*;

pub struct GameState {
    pub world: World,
    pub input_state: InputState,
    pub player: Option<Entity>,
    map_walls: Vec<u32>,
    map_floor: Vec<u32>,
    map_ceiling: Vec<u32>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            input_state: InputState::default(),
            player: None,
            map_walls: vec![
                1,1,1,1,1,1,1,1,
                1,0,1,0,0,0,0,1,
                1,0,1,0,1,5,0,1,
                1,0,1,0,4,0,0,1,
                1,0,0,0,1,0,0,1,
                1,0,1,4,1,0,0,1,
                1,0,0,0,0,0,0,1,
                1,1,1,1,1,1,1,1,
            ],
            map_floor: vec![
                0,0,0,0,0,0,0,0,
                0,2,0,2,2,2,2,0,
                0,2,0,2,0,0,2,0,
                0,2,0,2,0,2,2,0,
                0,2,2,2,0,2,2,0,
                0,2,0,0,0,2,2,0,
                0,2,2,2,2,2,2,2,
                0,0,0,0,0,0,0,0,
            ],
            map_ceiling: vec![
                0,0,0,0,0,0,0,0,
                0,3,0,3,3,3,3,0,
                0,3,0,3,0,0,3,0,
                0,3,0,3,0,3,3,0,
                0,3,3,3,0,3,3,0,
                0,3,0,0,0,3,3,0,
                0,3,3,3,3,3,3,3,
                0,0,0,0,0,0,0,0,
            ]
        }
    }

    pub fn start(&mut self) {
        self.create_player(1.5, 1.5, 0.0, 1.95);
        self.create_sprite(1.5, 6.5, 0.0, true, 0.0, 1.0, 1, 3);
        self.create_vent(4.0, 3.0, true, vent_hit);
    }

    pub fn update(&mut self, delta_time: f32) {
        PlayerRotationSystem(&mut self.world, &mut self.input_state);
        PlayerMovementSystem(&mut self.world, delta_time, &self.map_walls, &self.input_state);
        AnimatorSystem(&mut self.world, delta_time);
        InteractSystem(&mut self.world, &mut self.input_state, &mut self.player, &self.map_walls);
        VentSystem(&mut self.world, delta_time);
    }

    pub fn create_player(&mut self, x: f32, y: f32, angle: f32, speed: f32) {
        if self.player.is_none(){
            let player_entity = self.world.spawn((
                Position { x, y },
                Rotation { angle },
                Velocity { speed, dx: 0.0, dy: 0.0 },
                PlayerController { sensitivity: 0.0015 },
            ));

            self.player = Some(player_entity);
        }
    }

    pub fn get_camera_info(&mut self) -> Option<(f32, f32, f32)> {
        if let Some(player_id) = self.player {
            if let Ok((pos, rot, _)) = self.world.query_one_mut::<(&Position, &Rotation, &Velocity)>(player_id) {
                return Some((pos.x, pos.y, rot.angle));
            }
        }
        None
    }

    pub fn get_sprites(&mut self) -> Vec<SpriteInstance> {
        let mut sprite_instances: Vec<SpriteInstance> = self.world
            .query_mut::<(&Position, &Rotation, &Sprite)>()
            .into_iter()
            .filter(|(_, _, sprite)| sprite.is_visible)
            .map(|(pos, rot, sprite)| SpriteInstance{
                position: [pos.x, pos.y, sprite.z],
                direction: [rot.angle.cos(), rot.angle.sin()],
                scale: sprite.scale,
                atlas_index_front: sprite.atlas_index_front,
                atlas_index_back: sprite.atlas_index_back,
            })
            .collect();
        
        let (cam_x, cam_y, _) = self.get_camera_info().unwrap_or((0.0, 0.0, 0.0));

        sprite_instances.sort_by(|a, b| {
            let dist_a = (a.position[0] - cam_x).powi(2) + (a.position[1] - cam_y).powi(2);
            let dist_b = (b.position[0] - cam_x).powi(2) + (b.position[1] - cam_y).powi(2);
                        
            dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        sprite_instances
    }

    pub fn get_map_data(&self) -> Vec<u32> {
        let mut map_data = Vec::new();
        for i in 0..self.map_walls.len() {
            map_data.push(self.map_walls[i]);
            map_data.push(self.map_floor[i]);
            map_data.push(self.map_ceiling[i]);
            map_data.push(0);
        }
        map_data
    }

    pub fn create_sprite(&mut self, x: f32, y: f32, angle: f32, is_visible: bool, z: f32, scale: f32, atlas_index_front: u32, atlas_index_back: u32) {
        self.world.spawn((
            Position { x, y },
            Rotation { angle },
            Sprite { z, scale, is_visible, atlas_index_front, atlas_index_back },
        ));
    }

    pub fn create_vent(&mut self, x: f32, y: f32, is_enabled: bool, on_interact: InteractCallback) {
        self.world.spawn((
            Position { x, y },
            Interactable { is_enabled, on_interact },
        ));
    }
}

fn vent_hit(world: &mut World, player: &mut Option<Entity>, entity: Entity) {
    println!("Ventujem");
}