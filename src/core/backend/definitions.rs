#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum RenderPipelineType {
    Raycast,
    Blit,
    Sprite,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ComputePipelineType {
    Raycast,
}



#[derive(Eq, Hash, PartialEq)]
pub enum BindScope {
    Camera,
    Map,
    ComputeRayHits,
    RayHits,
    AtlasTexture,
    BlitTexture,
    AtlasSpriteTexture,
    SpriteInstances,
}