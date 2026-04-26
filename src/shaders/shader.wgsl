struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexPayload {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),   // Top vertex
        vec2<f32>(-0.5, -0.5), // Bottom left vertex
        vec2<f32>(0.5, -0.5)   // Bottom right vertex
    );
    let colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0), // Red
        vec3<f32>(0.0, 1.0, 0.0), // Green
        vec3<f32>(0.0, 0.0, 1.0)  // Blue
    );
    
    var out: VertexPayload;
    out.position = vec4<f32>(positions[i], 0.0, 1.0);
    out.color = colors[i];
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}