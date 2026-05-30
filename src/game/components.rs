use crate::game::definitions::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
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
    pub is_visible: bool,
    pub atlas_index_front: u32,
    pub atlas_index_back: u32,
}

pub struct Texture {
    pub atlas_index: u32,
}

pub struct TextureAnimation {
    pub frames: Vec<u16>,
    pub frame_duration: f32,
    pub playback_mode: PlaybackMode,
}

pub struct TextureAnimator {
    pub animations: HashMap<TextureAnimKey, TextureAnimation>,
    pub current_animation: TextureAnimKey,
    
    pub current_frame: usize,
    pub timer: f32,
    pub playback_state: PlaybackState,
    pub direction: AnimationDirection,
}

pub struct SpriteAnimation {
    pub frames_front: Vec<u16>,
    pub frames_back: Vec<u16>,
    pub frame_duration: f32,
    pub playback_mode: PlaybackMode,
}

pub struct SpriteAnimator {
    pub animations: HashMap<SpriteAnimKey, SpriteAnimation>,
    pub current_animation: SpriteAnimKey,
    
    pub current_frame: usize,
    pub timer: f32,
    pub playback_state: PlaybackState,
    pub direction: AnimationDirection,
}

pub struct Interactable {
    pub is_enabled: bool,
    pub on_interact: InteractCallback,
}

#[derive(Copy, Clone, Debug)]
pub struct Vent {
    pub is_open: bool,
    pub timer: f32,
    pub time_to_open: f32,
    pub orientation: VentOrientation,
    pub destinations: (Position, Position),
}