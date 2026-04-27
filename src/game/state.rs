use crate::game::player::Player;
use crate::game::input::InputState;

pub struct GameState {
    player: Player,
    pub input: InputState,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            input: InputState::new(),
        }
    }

    pub fn start(&mut self) {
        self.player.start();
    }

    pub fn update(&mut self, delta_time: f32) {
        self.player.update(delta_time, &mut self.input);
    }
}