
pub fn new_offscreen_texture (device: &wgpu::Device, height: u32, width: u32, format: wgpu::TextureFormat) -> (wgpu::TextureView, wgpu::Sampler) {
    let offscreen_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Offscreen Texture"),
        size: wgpu::Extent3d {width: width, height: height, depth_or_array_layers: 1},
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let offscreen_view = offscreen_texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("Offscreen Texture View"),
        ..Default::default()
    });

    let offscreen_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Offscreen Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    });

    (offscreen_view, offscreen_sampler)
}