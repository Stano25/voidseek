pub struct Vec3(pub f32, pub f32, pub f32);

pub const PLAYER_RADIUS: f32 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackMode {
    Once,
    Loop,
    PingPong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationDirection {
    Forward,
    Backward,
}