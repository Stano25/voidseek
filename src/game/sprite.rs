use crate::game::definitions::Vec3;

pub struct Sprite {
    pub position: Vec3,
    pub scale: f32,
    pub atlas_index: u32,
}