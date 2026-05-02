use std::env::current_dir;

pub struct Builder<'a> {
    rgba_data: Vec<u8>,
    texture_height: u32,
    texture_width: u32,
    layers: u32,
    pixel_format: wgpu::TextureFormat,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> Builder<'a> {
    pub fn new(device: &'a wgpu::Device, queue: &'a wgpu::Queue) -> Self {
        Self {
            rgba_data: Vec::new(),
            texture_height: 0,
            texture_width: 0,
            layers: 0,
            pixel_format: wgpu::TextureFormat::Rgba8UnormSrgb,
            device: device,
            queue: queue,
        }
    }

    pub fn add_textures(&mut self,folder_name: &[&str]) -> Result<(), String> {
        self.layers = folder_name.len() as u32;

        for name in folder_name {
            let mut filepath = current_dir().unwrap();
            filepath.push("src");
            filepath.push("assets");
            filepath.push(name);
            
            let img = image::open(filepath).map_err(|e| format!("Failed to open image: {}", e))?.to_rgba8();

            assert_eq!(
            img.dimensions(),
            (self.texture_width, self.texture_height),
            "Image {} does not have a {}x{} resolution!", name, self.texture_width, self.texture_height
            );

            self.rgba_data.extend_from_slice(img.as_raw());
        }
        Ok(())
    }

    pub fn set_texture_size(&mut self, width: u32, height: u32) {
        self.texture_width = width;
        self.texture_height = height;
    }

    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn build(&mut self, label: &str) -> wgpu::Texture {
        let texture_size = wgpu::Extent3d {
            width: self.texture_width,
            height: self.texture_height,
            depth_or_array_layers: self.layers,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.pixel_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.rgba_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.texture_width),
                rows_per_image: Some(self.texture_height),
            },
            texture_size,
        );

        self.rgba_data.clear();

        texture
    }
}