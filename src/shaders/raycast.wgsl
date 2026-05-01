struct Camera {
    position: vec2<f32>,
    direction: vec2<f32>,
    plane: vec2<f32>,
    resolution: vec2<f32>,
}

struct Map {
    width: u32,
    height: u32,
    tile_size: u32,
    wall_data: array<u32>,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<storage, read> map: Map;

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>, 
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexPayload {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0)
    );
    
    var out: VertexPayload;
    let pos = positions[i];
    
    out.position = vec4<f32>(pos, 0.0, 1.0);

    out.uv = vec2<f32>(
        pos.x * 0.5 + 0.5,
        1.0 - (pos.y * 0.5 + 0.5)
    );
    
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(in.uv.x, in.uv.y, 0.2, 1.0);
}