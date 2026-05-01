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
    // 1. Vypočítame X-ovú súradnicu lúča v priestore kamery (-1 je ľavý okraj, 1 je pravý)
    let camera_x = 2.0 * in.uv.x - 1.0;
    
    // 2. Vektor smeru aktuálneho lúča
    let ray_dir = vec2<f32>(
        camera.direction.x + camera.plane.x * camera_x,
        camera.direction.y + camera.plane.y * camera_x
    );

    // 3. Normalizujeme pozíciu kamery na mriežku mapy (ak je pozícia v pixeloch)
    // Tvoj C++ kód posúval >>6 (čo je delenie 64). Takto dostaneme desatinnú pozíciu v blokoch mapy.
    let ray_pos = camera.position / f32(map.tile_size);
    
    // Aktuálny štvorec v mape (celočíselné súradnice)
    var map_pos = vec2<i32>(floor(ray_pos));

    // 4. Príprava DDA algoritmu (určuje, ako rýchlo lúč pretína X-ové a Y-ové čiary gridu)
    let delta_dist = vec2<f32>(
        abs(1.0 / (ray_dir.x + 1e-20)), // 1e-20 chráni proti deleniu nulou
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

    // 5. Hlavný cyklus posielania lúča (DDA Loop)
    var hit = false;
    var side = 0; // 0 = vertikálna stena (dofV), 1 = horizontálna stena (dofH)
    
    for (var i = 0; i < 50; i++) { // Obmedzíme maximálnu dohľadnosť
        // Posúvame lúč na najbližšiu čiaru mriežky mapy
        if (side_dist.x < side_dist.y) {
            side_dist.x += delta_dist.x;
            map_pos.x += step_dir.x;
            side = 0;
        } else {
            side_dist.y += delta_dist.y;
            map_pos.y += step_dir.y;
            side = 1;
        }

        // Kontrola, či sme stále v mape
        if (map_pos.x >= 0 && map_pos.x < i32(map.width) &&
            map_pos.y >= 0 && map_pos.y < i32(map.height)) {
            
            // mapIndex
            let map_index = u32(map_pos.y) * map.width + u32(map_pos.x);
            if (map.wall_data[map_index] == 1u) {
                hit = true;
                break;
            }
        }
    }

    // 6. Ak lúč nič netrafil, nakreslíme len podlahu alebo strop
    let frag_y_pixels = in.uv.y * camera.resolution.y; // Y pozícia v pixeloch (napr. 0-270)
    
    if (!hit) {
        if (in.uv.y < 0.5) {
            return vec4<f32>(0.0, 1.0, 1.0, 1.0); // Cyan Strop
        } else {
            return vec4<f32>(0.0, 0.0, 1.0, 1.0); // Modrá Podlaha 
        }
    }

    // 7. Oprava Fisheye efektu a výpočet kolmej vzdialenosti k stene
    var perp_wall_dist: f32;
    if (side == 0) {
        perp_wall_dist = (f32(map_pos.x) - ray_pos.x + (1.0 - f32(step_dir.x)) / 2.0) / ray_dir.x;
    } else {
        perp_wall_dist = (f32(map_pos.y) - ray_pos.y + (1.0 - f32(step_dir.y)) / 2.0) / ray_dir.y;
    }

    // 8. Výpočet výšky čiary (wall line height) 
    let line_height = camera.resolution.y / perp_wall_dist;
    
    let draw_start = -line_height / 2.0 + camera.resolution.y / 2.0;
    let draw_end = line_height / 2.0 + camera.resolution.y / 2.0;

    // 9. Samotné kreslenie pixelu pre danú Y súradnicu
    if (frag_y_pixels < draw_start) {
        return vec4<f32>(0.0, 1.0, 1.0, 1.0); // Cyan Strop nad stenou
    } else if (frag_y_pixels > draw_end) {
        return vec4<f32>(0.0, 0.0, 1.0, 1.0); // Modrá Podlaha pod stenou
    } else {
        // Pixel je vnútri steny! Zistíme, aká má byť farba podľa toho, z ktorej strany sme stenu trafili
        if (side == 0) {
            return vec4<f32>(0.0, 0.8, 0.0, 1.0); // Zelená
        } else {
            return vec4<f32>(0.0, 0.6, 0.0, 1.0); // Tmavšia Zelená
        }
    }
}