use crate::{MAX_MAP_HEIGHT, MAX_MAP_TILES, MAX_MAP_WIDTH, game::state::GameState};
use crate::game::definitions::*;
use hecs::{World};
use wgpu::wgc::validation;
use crate::game::state::vent_hit;

pub struct MapManager {
    walls_data: Vec<u16>,
    floor_data: Vec<u16>,
    ceiling_data: Vec<u16>,

    dirty_tiles: Vec<(u32, u32, u32, u32)>,
}

impl Default for MapManager {
    fn default() -> Self {
        Self {
            walls_data: vec![0; MAX_MAP_TILES as usize],
            floor_data: vec![0; MAX_MAP_TILES as usize],
            ceiling_data: vec![0; MAX_MAP_TILES as usize],
            dirty_tiles: Vec::new(),
        }
     }
}

impl MapManager {
    pub fn load_from_layout(&mut self, layout: &[&str], world: &mut World) {
        let mut vents_to_place: Vec<(u32, u32)> = Vec::new();

        for (y, &tile) in layout.iter().enumerate() {
            if y >= MAX_MAP_HEIGHT as usize { break; }
            for (x, ch) in tile.chars().enumerate() {
                if x >= MAX_MAP_WIDTH as usize { break; }
                let i = y * MAX_MAP_WIDTH as usize + x;
                self.walls_data[i] = 0;
                self.floor_data[i] = 2;
                self.ceiling_data[i] = 3;

                if ch == '1' {
                    self.walls_data[i] = 1;
                } else if ch == 'V' {
                    vents_to_place.push((x as u32, y as u32));
                }
            }
        }

        for (x, y) in vents_to_place {
            let (vent_valid, vent_orientation) = self.check_vent_placement(x, y);
            let i = (y * MAX_MAP_WIDTH  + x) as usize;
            if vent_valid {
                self.walls_data[i] = 14;
                GameState::create_vent(world, x as f32, y as f32, true, vent_hit, vent_orientation);
            }
            else {
                self.walls_data[i] = 1;
            }
        }
    }

    fn check_vent_placement(&self, x: u32, y: u32) -> (bool, VentOrientation) {
        let (top_index, bottom_index, left_index, right_index) = {
            let top = if y > 0 { self.walls_data[((y - 1) * MAX_MAP_WIDTH + x) as usize] } else { 1 };
            let bottom = if y < MAX_MAP_HEIGHT - 1 { self.walls_data[((y + 1) * MAX_MAP_WIDTH + x) as usize] } else { 1 };
            let left = if x > 0 { self.walls_data[(y * MAX_MAP_WIDTH + (x - 1)) as usize] } else { 1 };
            let right = if x < MAX_MAP_WIDTH - 1 { self.walls_data[(y * MAX_MAP_WIDTH + (x + 1)) as usize] } else { 1 };
            (top, bottom, left, right)
        };

        let mut orientation: VentOrientation = VentOrientation::None;
        let mut valid_placement = false;

        if top_index == 0 && bottom_index == 0 && left_index != 0 && right_index != 0 {
            orientation = VentOrientation::Vertical;
            valid_placement = true;
        } else if left_index == 0 && right_index == 0 && top_index != 0 && bottom_index != 0 {
            orientation = VentOrientation::Horizontal;
            valid_placement = true;
        }
        
        (valid_placement, orientation)
    }

    pub fn set_wall(&mut self, x: u32, y: u32, texture_id: u16) {
        if x < MAX_MAP_WIDTH && y < MAX_MAP_HEIGHT {
            let index = (y * MAX_MAP_WIDTH + x) as usize;
            self.walls_data[index] = texture_id;
            self.dirty_tiles.push((index as u32, texture_id as u32, self.floor_data[index] as u32, self.ceiling_data[index] as u32));
        }
    }

    pub fn get_walls_data(&self) -> &[u16] {
        &self.walls_data
    }

    pub fn get_floor_data(&self) -> &[u16] {
        &self.floor_data
    }

    pub fn get_ceiling_data(&self) -> &[u16] {
        &self.ceiling_data
    }

    pub fn get_dirty_tiles(&self) -> &[(u32, u32, u32, u32)] {
        &self.dirty_tiles
    }
}