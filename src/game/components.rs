pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Rotation {
    pub angle: f32,
}

pub struct Velocity{
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