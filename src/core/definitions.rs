#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PipelineType {
    Raycast,
    Blit,
}

#[derive(Eq, Hash, PartialEq)]
pub enum BindScope {
    Camera,
    Map,
    BlitTexture,
}