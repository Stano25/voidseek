struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>, // Zmenili sme farby na UV súradnice pre textúru
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexPayload {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0), // Ľavý dolný roh obrazovky
        vec2<f32>( 3.0, -1.0), // Pravý dolný roh (úplne mimo monitora vpravo)
        vec2<f32>(-1.0,  3.0)  // Ľavý horný roh (úplne mimo monitora hore)
    );
    
    var out: VertexPayload;
    let pos = positions[i];
    
    out.position = vec4<f32>(pos, 0.0, 1.0);
    
    // Prepočet obrazovky (-1 až 1) na UV súradnice (0 až 1)
    // Toto budeš nevyhnutne potrebovať v ďalšom kroku pre načítanie textúry!
    // U(x) ide zľava doprava, V(y) ide zhora nadol.
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