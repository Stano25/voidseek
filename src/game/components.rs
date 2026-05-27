use crate::game::definitions::*;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Rotation {
    pub angle: f32,
}

pub struct Velocity {
    pub speed: f32,
    pub dx: f32,
    pub dy: f32,
}

pub struct PlayerController {
    pub sensitivity: f32,
}

pub struct Sprite {
    pub z: f32,
    pub scale: f32,
    pub atlas_index_front: u32,
    pub atlas_index_back: u32,
}

pub struct Texture {
    pub atlas_index: u32,
}

pub struct TextureAnimator {
    pub frames: Vec<u16>,
    pub current_frame: usize,
    pub frame_duration: f32,
    pub timer: f32,
    pub playback_state: PlaybackState,
    pub playback_mode: PlaybackMode,
    pub is_reversed: bool,
}

pub struct SpriteAnimator {
    pub frames_front: Vec<u16>,
    pub frames_back: Vec<u16>,
    pub current_frame: usize,
    pub frame_duration: f32,
    pub timer: f32,
    pub playback_state: PlaybackState,
    pub playback_mode: PlaybackMode,
    pub is_reversed: bool,
}