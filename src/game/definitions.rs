// --- Constants ---
pub const PLAYER_RADIUS: f32 = 10.0;

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
pub enum DoorAnim {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureAnimKey {
    Door(DoorAnim),
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

