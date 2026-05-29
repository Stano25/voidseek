use hecs::{Entity, World};
use crate::game::state::GameState;

// --- Constants ---
pub const PLAYER_RADIUS: f32 = 10.0;
pub const INTERACT_DISTANCE: f32 = 1.5;

// --- Animation Playback Control ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Playing,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackMode {
    Once,
    Loop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationDirection {
    Forward,
    Backward,
}

// --- Texture Animations ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VentAnim {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DoorAnim {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureAnimKey {
    Door(DoorAnim),
    Vent(VentAnim),
}

// --- Sprite Animations ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunnerAnim {
    Idle,
    Walk,
    Pickup,
    Hurt,
    Death,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChaserAnim {
    Idle,
    Walk,
    Attack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteAnimKey {
    Runner(RunnerAnim),
    Chaser(ChaserAnim),
}

pub type InteractCallback = fn(&mut World, &mut Option<Entity>, Entity);