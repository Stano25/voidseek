use crate::game::player::Player;
use crate::game::input::InputState;
use crate::game::sprite::{Sprite};
use crate::game::definitions::Vec3;

pub struct GameState {
    player: Player,
    pub input: InputState,
    map_walls: Vec<u32>,
    map_floor: Vec<u32>,
    map_ceiling: Vec<u32>,
    pub sprites: Vec<Sprite>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            input: InputState::new(),
            map_walls: vec![
                1,1,1,1,1,1,1,1,
                1,0,1,0,0,0,0,1,
                1,0,1,0,1,1,0,1,
                1,0,1,0,1,0,0,1,
                1,0,0,0,1,0,0,1,
                1,0,1,1,1,0,0,1,
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
            ],
            sprites: vec![
                Sprite {
                    position: Vec3(1.5, 6.5, 0.0),
                    scale: 1.0,
                    atlas_index: 1,
                },
            ],
        }
    }

    pub fn start(&mut self) {
        self.player.start();
    }

    pub fn update(&mut self, delta_time: f32) {
        self.player.update(delta_time, &mut self.input, &self.map_walls);
    }

    pub fn camera_pose(&self) -> (f32, f32, f32) {
        let (x, y) = self.player.position();
        (x, y, self.player.angle())
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

    pub fn get_sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }
}