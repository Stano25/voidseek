struct Camera {
    position: vec2<f32>,
    direction: vec2<f32>,
    plane: vec2<f32>,
    resolution: vec2<f32>,
}

struct MapSettings {
    width: u32,
    height: u32,
    tile_size: u32,
    
    _padding: u32, 
}

struct Tile {
    wall_texture_id: u32,
    floor_texture_id: u32,
    ceiling_texture_id: u32,
    
    _padding: u32,
}

// --- Bind Groups ---

// Group 0: Kamera
@group(0) @binding(0) var<uniform> camera: Camera;

// Group 1: Všetko pre Mapu
@group(1) @binding(0) var<storage, read> map_data: array<Tile>;
@group(1) @binding(1) var<uniform> map_settings: MapSettings;

// Group 2: Textúry
@group(2) @binding(0) var texture_atlas: texture_2d_array<f32>;
@group(2) @binding(1) var texture_sampler: sampler;

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>, 
}

// ... Vertex shader (vs_main) zostáva úplne rovnaký ...
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
    out.uv = vec2<f32>(pos.x * 0.5 + 0.5, 1.0 - (pos.y * 0.5 + 0.5));
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    let camera_x = 2.0 * in.uv.x - 1.0;
    
    let ray_dir = vec2<f32>(
        camera.direction.x + camera.plane.x * camera_x,
        camera.direction.y + camera.plane.y * camera_x
    );

    let ray_pos = camera.position / f32(map_settings.tile_size);
    var map_pos = vec2<i32>(floor(ray_pos));

    let delta_dist = vec2<f32>(
        abs(1.0 / (ray_dir.x + 1e-20)),
        abs(1.0 / (ray_dir.y + 1e-20))
    );

    var step_dir = vec2<i32>(0, 0);
    var side_dist = vec2<f32>(0.0, 0.0);

    if (ray_dir.x < 0.0) {
        step_dir.x = -1;
        side_dist.x = (ray_pos.x - f32(map_pos.x)) * delta_dist.x;
    } else {
        step_dir.x = 1;
        side_dist.x = (f32(map_pos.x) + 1.0 - ray_pos.x) * delta_dist.x;
    }

    if (ray_dir.y < 0.0) {
        step_dir.y = -1;
        side_dist.y = (ray_pos.y - f32(map_pos.y)) * delta_dist.y;
    } else {
        step_dir.y = 1;
        side_dist.y = (f32(map_pos.y) + 1.0 - ray_pos.y) * delta_dist.y;
    }

    var hit = false;
    var side = 0;
    var hit_wall_tex_id = 0u;

    for (var i = 0; i < 50; i++) {
        if (side_dist.x < side_dist.y) {
            side_dist.x += delta_dist.x;
            map_pos.x += step_dir.x;
            side = 0;
        } else {
            side_dist.y += delta_dist.y;
            map_pos.y += step_dir.y;
            side = 1;
        }

        if (map_pos.x >= 0 && map_pos.x < i32(map_settings.width) &&
            map_pos.y >= 0 && map_pos.y < i32(map_settings.height)) {
            
            let map_index = u32(map_pos.y) * map_settings.width + u32(map_pos.x);
            let tile = map_data[map_index];
            
            if (tile.wall_texture_id > 0u) {
                hit = true;
                hit_wall_tex_id = tile.wall_texture_id;
                break;
            }
        }
    }

    var perp_wall_dist: f32;
    if (side == 0) {
        perp_wall_dist = (f32(map_pos.x) - ray_pos.x + (1.0 - f32(step_dir.x)) / 2.0) / ray_dir.x;
    } else {
        perp_wall_dist = (f32(map_pos.y) - ray_pos.y + (1.0 - f32(step_dir.y)) / 2.0) / ray_dir.y;
    }

    // --- ZVÄČŠENIE STENY NA KOCKU ---
    // Korekcia zobrazenia (16:9). Týmto sa stena natiahne do výšky, aby bola štvorcová
    let aspect_fix = 1.3333; 
    let line_height = (camera.resolution.y * aspect_fix) / perp_wall_dist;
    
    let draw_start = -line_height / 2.0 + camera.resolution.y / 2.0;
    let draw_end = line_height / 2.0 + camera.resolution.y / 2.0;
    let frag_y_pixels = in.uv.y * camera.resolution.y;

    // --- RENDER PODLAHY A STROPU ---
    if (frag_y_pixels < draw_start) {
        // Render STROPU (Ceiling casting)
        let p = (camera.resolution.y / 2.0) - frag_y_pixels;
        // Výpočet vzdialenosti k stropu, prispôsobený pre aspect ratio
        let row_dist = (camera.resolution.y * aspect_fix / 2.0) / p; 

        let floor_pos = ray_pos + row_dist * ray_dir;
        let map_x = i32(floor(floor_pos.x));
        let map_y = i32(floor(floor_pos.y));

        if (map_x >= 0 && map_x < i32(map_settings.width) &&
            map_y >= 0 && map_y < i32(map_settings.height)) {
            
            let map_index = u32(map_y) * map_settings.width + u32(map_x);
            let tile = map_data[map_index];
            
            if (tile.ceiling_texture_id > 0u) {
                let tex_x = 1.0 - fract(floor_pos.x);
                let tex_y = 1.0 - fract(floor_pos.y);
                let atlas_layer = i32(tile.ceiling_texture_id) - 1;
                
                let color = textureSampleLevel(texture_atlas, texture_sampler, vec2<f32>(tex_x, tex_y), atlas_layer, 0.0);
                return vec4<f32>(color.rgb * 0.5, 1.0); // Strop je trošku tmavší
            }
        }
        return vec4<f32>(0.1, 0.1, 0.1, 1.0);

    } else if (frag_y_pixels > draw_end) {
        // Render PODLAHY (Floor casting)
        let p = frag_y_pixels - (camera.resolution.y / 2.0);
        let row_dist = (camera.resolution.y * aspect_fix / 2.0) / p;

        let floor_pos = ray_pos + row_dist * ray_dir;
        let map_x = i32(floor(floor_pos.x));
        let map_y = i32(floor(floor_pos.y));

        if (map_x >= 0 && map_x < i32(map_settings.width) &&
            map_y >= 0 && map_y < i32(map_settings.height)) {
            
            let map_index = u32(map_y) * map_settings.width + u32(map_x);
            let tile = map_data[map_index];
            
            if (tile.floor_texture_id > 0u) {
                let tex_x = fract(floor_pos.x);
                let tex_y = 1.0 - fract(floor_pos.y);
                let atlas_layer = i32(tile.floor_texture_id) - 1;
                
                let color = textureSampleLevel(texture_atlas, texture_sampler, vec2<f32>(tex_x, tex_y), atlas_layer, 0.0);
                return vec4<f32>(color.rgb * 0.7, 1.0); // Podlaha
            }
        }
        return vec4<f32>(0.2, 0.2, 0.2, 1.0);

    } else {
        // Render STIEN (Wall casting)
        if (!hit) { return vec4<f32>(0.0, 0.0, 0.0, 1.0); }

        var wall_x: f32;
        if (side == 0) { wall_x = ray_pos.y + perp_wall_dist * ray_dir.y; } 
        else           { wall_x = ray_pos.x + perp_wall_dist * ray_dir.x; }
        wall_x = wall_x - floor(wall_x);

        var tex_x = wall_x;
        if (side == 0 && ray_dir.x < 0.0) { tex_x = 1.0 - tex_x; }
        if (side == 1 && ray_dir.y > 0.0) { tex_x = 1.0 - tex_x; }

        // CLAMP: Zabraňuje vynechávaniu najvyšších a najnižších pixelov
        let tex_y = clamp((frag_y_pixels - draw_start) / line_height, 0.0, 1.0);
        
        let atlas_layer = i32(hit_wall_tex_id) - 1; 
        var color = textureSampleLevel(texture_atlas, texture_sampler, vec2<f32>(tex_x, tex_y), atlas_layer, 0.0);

        if (side == 1) {
            color = vec4<f32>(color.rgb * 0.35, color.a);
        }
        return color;
    }
}