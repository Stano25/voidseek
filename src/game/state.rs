use crate::game::input::InputState;
use crate::core::renderer::{SpriteInstance};
use hecs::{Entity, World};
use crate::game::components::{Position, Rotation, Velocity, PlayerController, Sprite, Interactable, TextureAnimator, Vent};
use crate::game::systems::*;
use crate::game::definitions::*;
use crate::game::map::MapManager;
use crate::{TILE_SIZE};

pub struct GameState {
    pub world: World,
    pub input_state: InputState,
    pub player: Option<Entity>,
    map: MapManager,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            input_state: InputState::default(),
            player: None,
            map: MapManager::default(),
        }
    }

    pub fn start(&mut self) {
        self.map.load_from_layout(&[
            "11111111",
            "1.1....1",
            "1.1.11.1",
            "1.1.1..1",
            "1...V..1",
            "1.111.11",
            "1......1",
            "11111111",
        ], &mut self.world);
        self.create_player(1.5, 1.5, 0.0, 1.95);
        GameState::create_sprite(&mut self.world,1.5, 6.5, 0.0, true, 0.0, 1.0, 1, 3);
    }

    pub fn update(&mut self, delta_time: f32) {
        PlayerRotationSystem(&mut self.world, &mut self.input_state);
        PlayerMovementSystem(&mut self.world, delta_time, &self.map.get_walls_data(), &self.input_state);
        AnimatorSystem(&mut self.world, delta_time);
        InteractSystem(&mut self.world, &mut self.input_state, &mut self.player, &self.map.get_walls_data());
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
        let walls_data = self.map.get_walls_data();
        let floor_data = self.map.get_floor_data();
        let ceiling_data = self.map.get_ceiling_data();

        for i in 0..walls_data.len() {
            map_data.push(walls_data[i] as u32);
            map_data.push(floor_data[i] as u32);
            map_data.push(ceiling_data[i] as u32);
            map_data.push(0);
        }
        map_data
    }

    pub fn create_sprite(world: &mut World, x: f32, y: f32, angle: f32, is_visible: bool, z: f32, scale: f32, atlas_index_front: u32, atlas_index_back: u32) {
        world.spawn((
            Position { x, y },
            Rotation { angle },
            Sprite { z, scale, is_visible, atlas_index_front, atlas_index_back },
        ));
    }

    pub fn create_vent(world: &mut World, x: f32, y: f32, is_enabled: bool, on_interact: InteractCallback, orientation: VentOrientation) {
        let inverted_size = 1.0 / TILE_SIZE as f32;

        let player_rad = PLAYER_RADIUS * inverted_size;
        let vent_offset = VENT_OFFSET * inverted_size;

        let (vent_center_x, vent_center_y) = (x + 0.5, y + 0.5);

        let (pos_1, pos_2) = match orientation {
            VentOrientation::Vertical => (Position { x: vent_center_x, y: vent_center_y - 0.5- player_rad - vent_offset}, Position { x: vent_center_x, y: vent_center_y + 0.5 + player_rad + vent_offset}),
            VentOrientation::Horizontal => (Position{ x: vent_center_x - 0.5 - player_rad - vent_offset, y: vent_center_y }, Position{ x: vent_center_x + 0.5 + player_rad + vent_offset, y: vent_center_y }),
            VentOrientation::None => { return; }
        };
        //println!("Creating vent at ({}, {}) with orientation {:?} and destinations: ({}, {}) and ({}, {})", x, y, orientation, pos_1.x, pos_1.y, pos_2.x, pos_2.y);
        world.spawn((
            Position { x, y },
            Interactable { is_enabled, on_interact },
            Vent { is_open: true, timer: 0.0, time_to_open: TIME_TO_OPEN_VENT, orientation, destinations: (pos_1, pos_2) },
        ));
    }

    pub fn destroy_all_vents(world: &mut World) {
        let vent_entities: Vec<Entity> = world
            .query::<(&Entity, &Vent)>() 
            .iter()
            .map(|(entity, _vent)| *entity) 
            .collect();

        for entity in vent_entities {
            world.despawn(entity).ok();
        }
    }
}

pub fn vent_hit(world: &mut World, player: &mut Option<Entity>, entity: Entity) {
    let player = if let Some(player) = player { *player } else { return; };

    let mut vent_querys = None;
    let mut should_vent_close = false;
    if let Ok(mut vent) = world.query_one_mut::<(&Vent)>(entity) {
        if !vent.is_open{ return; }
        vent_querys = Some(*vent);
    }

    if let Some(vent) = vent_querys {
        if let Ok(mut player_pos) = world.query_one_mut::<&mut Position>(player) {
            let first_point_distance = {
                let dx = player_pos.x - vent.destinations.0.x;
                let dy = player_pos.y - vent.destinations.0.y;
                (dx * dx + dy * dy).sqrt()
            };

            let second_point_distance = {
                let dx = player_pos.x - vent.destinations.1.x;
                let dy = player_pos.y - vent.destinations.1.y;
                (dx * dx + dy * dy).sqrt()
            };

            if first_point_distance < second_point_distance {
                player_pos.x = vent.destinations.1.x;
                player_pos.y = vent.destinations.1.y;
            } else {
                player_pos.x = vent.destinations.0.x;
                player_pos.y = vent.destinations.0.y;
            }
            should_vent_close = true;
        }
    }

    if let Ok(vent) = world.query_one_mut::<(&mut Vent)>(entity) && should_vent_close {
        vent.is_open = false;
    }
}