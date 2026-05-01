use crate::game::player::Player;
use crate::game::input::InputState;

pub struct GameState {
    player: Player,
    pub input: InputState,
    map: Vec<u32>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            input: InputState::new(),
            map: vec![
                1,1,1,1,1,1,1,1,
                1,0,1,0,0,0,0,1,
                1,0,1,0,0,0,0,1,
                1,0,1,0,0,0,0,1,
                1,0,0,0,0,0,0,1,
                1,0,0,0,0,1,0,1,
                1,0,0,0,0,0,0,1,
                1,1,1,1,1,1,1,1,
            ],
        }
    }

    pub fn start(&mut self) {
        self.player.start();
    }

    pub fn update(&mut self, delta_time: f32) {
        self.player.update(delta_time, &mut self.input, &self.map);
    }

    pub fn camera_pose(&self) -> (f32, f32, f32) {
        let (x, y) = self.player.position();
        (x, y, self.player.angle())
    }

    pub fn get_map_data(&self) -> &[u32] {
        &self.map
    }
}