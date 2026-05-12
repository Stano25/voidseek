use std::sync::Arc;
use winit::window::{Window};
use crate::core::backend::{pipeline,
    compute_pipeline,
    bind_group_layout,
    atlas,
    definitions::{RenderPipelineType, ComputePipelineType, BindScope},
    texture::{new_offscreen_texture}
    };
use wgpu::util::DeviceExt;
use std::collections::HashMap;
use crate::{MAX_MAP_TILES, TILE_SIZE, MAX_MAP_WIDTH, MAX_MAP_HEIGHT};

const RENDER_WIDTH: u32 = 960;
const RENDER_HEIGHT: u32 = 540;

struct CameraResources {
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

struct MapResources {
    bind_group: wgpu::BindGroup,
    data_buffer: wgpu::Buffer,
    settings_buffer: wgpu::Buffer,
}

struct AtlasResources {
    bind_group: wgpu::BindGroup,
    texture_view: wgpu::TextureView,
}

struct BlitResources {
    offscreen_texture: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

pub struct WgpuState {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    window: Arc<Window>,
    render_pipelines: HashMap<RenderPipelineType, wgpu::RenderPipeline>,
    compute_pipelines: HashMap<ComputePipelineType, wgpu::ComputePipeline>,
    bind_group_layouts: HashMap<BindScope, wgpu::BindGroupLayout>,
    camera_resources: CameraResources,
    map_resources: MapResources,
    atlas_resources: AtlasResources,
    blit_resources: BlitResources,
    compute_bind_group: wgpu::BindGroup,
    ray_hits_bind_group: wgpu::BindGroup,
    ray_hits_buffer: wgpu::Buffer,
}

impl WgpuState {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            flags: Default::default(),
            backend_options: Default::default(),
            memory_budget_thresholds: Default::default(),
            display: Default::default(),
        };
        let instance = wgpu::Instance::new(instance_descriptor);

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format  = surface_capabilities.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next().unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        // Vytvorenie offscreen textúry
        let offscreen_format = wgpu::TextureFormat::Rgba8Unorm;

        let bind_group_layouts = Self::build_bind_groups_layouts(&device);

        let render_pipelines = Self::build_render_pipelines(&device, &config, offscreen_format, &bind_group_layouts);

        let compute_pipelines = Self::build_compute_pipelines(&device, &bind_group_layouts);
        // =====================================================================
        // Inicializácia offscreen textúry a jej bind group
        // =====================================================================
        let (offscreen_view, offscreen_sampler) = new_offscreen_texture(&device, RENDER_WIDTH, RENDER_HEIGHT, offscreen_format);

        let blit_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Blit Bind Group"),
            layout: &bind_group_layouts[&BindScope::BlitTexture],
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&offscreen_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&offscreen_sampler),
                },
            ],
        });

        let blit_resources = BlitResources {
            offscreen_texture: offscreen_view,
            bind_group: blit_bind_group,
        };

        // =====================================================================
        // Inicializácia atlasu textúr
        // =====================================================================
        let atlas_texture = Self::create_atlas_texture(&device, &queue, wgpu::TextureFormat::Rgba8UnormSrgb);

        let atlas_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Texture Array View"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let atlas_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Retro Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let atlas_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Array Bind Group"),
            layout: &bind_group_layouts[&BindScope::AtlasTexture],
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&atlas_sampler),
                },
            ],
        });

        let atlas_resources = AtlasResources {
            bind_group: atlas_bind_group,
            texture_view: atlas_view,
        };

        let ray_hits_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Ray Hits Buffer"),
            size: (RENDER_WIDTH * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        // =====================================================================
        // Inicializácia kamery s defaultnými hodnotami
        // =====================================================================
        let camera_data:[f32; 8] =[0.0; 8]; 
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&camera_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layouts[&BindScope::Camera],
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let camera_resources = CameraResources {
            bind_group: camera_bind_group,
            buffer: camera_buffer,
        };

        // =====================================================================
        // Inicializácia mapy s defaultnými hodnotami
        // =====================================================================
        let mut map_settings_data: [u32; 4] = [MAX_MAP_WIDTH, MAX_MAP_HEIGHT, TILE_SIZE, 5];

        let map_settings_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Settings Buffer"),
            contents: bytemuck::cast_slice(&map_settings_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let map_data: [u32; (MAX_MAP_TILES*4) as usize] = [0; (MAX_MAP_TILES*4) as usize];
        let map_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Buffer"),
            contents: bytemuck::cast_slice(&map_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let map_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Map Bind Group"),
            layout: &bind_group_layouts[&BindScope::Map],
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: map_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: map_settings_buffer.as_entire_binding(),
                }
            ],
        });

        let ray_hits_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Ray Hits Bind Group"),
            layout: &bind_group_layouts[&BindScope::RayHits],
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: ray_hits_buffer.as_entire_binding() },
            ],
        });

        let map_resources = MapResources {
            bind_group: map_bind_group,
            data_buffer: map_buffer,
            settings_buffer: map_settings_buffer,
        };

        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &bind_group_layouts[&BindScope::ComputeRayHits],
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: camera_resources.buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: map_resources.settings_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: map_resources.data_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: ray_hits_buffer.as_entire_binding() },
            ],
        });

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size: (size.width, size.height),
            window,
            render_pipelines,
            bind_group_layouts,
            camera_resources,
            map_resources,
            atlas_resources,
            blit_resources,
            compute_pipelines,
            ray_hits_buffer,
            ray_hits_bind_group,
            compute_bind_group,
        }
    }

    pub fn render(&mut self) {
        let current_surface_texture = self.surface.get_current_texture();
        let drawable = match current_surface_texture {
            wgpu::CurrentSurfaceTexture::Success(frame) | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Timeout => {
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.resize(self.size.0, self.size.1);
                return;
            }
            _ => return,
        };

        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);
        
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        {
            let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Raycast Compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipelines[&ComputePipelineType::Raycast]);
            compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);

            let dispatch_count = (RENDER_WIDTH + 63) / 64;
            compute_pass.dispatch_workgroups(dispatch_count, 1, 1);
        }

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Raycast Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.blit_resources.offscreen_texture,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            let render_pipeline = self.render_pipelines.get(&RenderPipelineType::Raycast).unwrap();
            render_pass.set_pipeline(render_pipeline);
            render_pass.set_bind_group(0, &self.camera_resources.bind_group, &[]);
            render_pass.set_bind_group(1, &self.map_resources.bind_group, &[]);
            render_pass.set_bind_group(2, &self.atlas_resources.bind_group, &[]);
            render_pass.set_bind_group(3, &self.ray_hits_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        //Kreslíme texturu na okno
        {
            let mut blit_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Blit Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &image_view, // Kreslíme do okna
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            let blit_pipeline = self.render_pipelines.get(&RenderPipelineType::Blit).unwrap();
            blit_pass.set_pipeline(blit_pipeline);
            blit_pass.set_bind_group(0, &self.blit_resources.bind_group, &[]);
            blit_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.size = (new_width, new_height);
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn build_bind_groups_layouts(device: &wgpu::Device) -> HashMap<BindScope, wgpu::BindGroupLayout> {
        let mut layouts = HashMap::new();
        let mut builder = bind_group_layout::Builder::new(device);
        let mut layout: wgpu::BindGroupLayout;

        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        layout = builder.build("Camera Bind Group Layout");
        layouts.insert(BindScope::Camera, layout);

        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        layout = builder.build("Map Bind Group Layout");
        layouts.insert(BindScope::Map, layout);

        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2Array,
                multisampled: false,
            },
            count: None,
        });
        
        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });

        layout = builder.build("Atlas Bind Group Layout");
        layouts.insert(BindScope::AtlasTexture, layout);
        
        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        });
        
        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
        
        layout = builder.build("Blit Bind Group Layout");
        layouts.insert(BindScope::BlitTexture, layout);

        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        layout = builder.build("Ray Hits Fragment Layout");
        layouts.insert(BindScope::RayHits, layout);

        builder.add_entry(wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
        });
        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        builder.add_entry(wgpu::BindGroupLayoutEntry {
            binding: 3,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        layout = builder.build("Compute Ray Hits Layout");
        layouts.insert(BindScope::ComputeRayHits, layout);

        layouts
    }

    fn build_render_pipelines(device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        offscreen_format: wgpu::TextureFormat,
        bind_group_layouts: &HashMap<BindScope, wgpu::BindGroupLayout>)
        -> HashMap<RenderPipelineType, wgpu::RenderPipeline> {

        let mut pipelines = HashMap::new();
        let mut builder = pipeline::Builder::new(device);
        let mut pipeline: wgpu::RenderPipeline;
        builder.set_shader_module("raycast_retro.wgsl", "vs_main", "fs_main");
        builder.set_pixel_format(offscreen_format);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::Camera]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::Map]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::AtlasTexture]); 
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::RayHits]);
        pipeline = builder.build("Raycast Pipeline");
        
        pipelines.insert(RenderPipelineType::Raycast, pipeline);

        let mut blit_builder = pipeline::Builder::new(device);
        blit_builder.set_shader_module("blit.wgsl", "vs_main", "fs_main");
        blit_builder.set_pixel_format(config.format);
        blit_builder.add_bind_group_layout(&bind_group_layouts[&BindScope::BlitTexture]);
        pipelines.insert(RenderPipelineType::Blit, blit_builder.build("Blit Pipeline"));

        pipelines
    }

    fn create_atlas_texture(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> wgpu::Texture {
        let mut builder = atlas::Builder::new(device, queue);
        builder.set_pixel_format(format);
        builder.add_textures(&["Wall-Texture.png", "Floor-Texture.png", "Ceiling-Texture.png"]).expect("Failed to add textures to atlas");
        builder.build("Atlas Texture")
    }

    fn build_compute_pipelines(device: &wgpu::Device, bind_group_layouts: &HashMap<BindScope, wgpu::BindGroupLayout>) -> HashMap<ComputePipelineType, wgpu::ComputePipeline> {
        let mut compute_pipelines = HashMap::new();
        let mut builder = compute_pipeline::Builder::new(device);
        builder.set_shader_module("raycast_compute.wgsl", "cs_main");
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::ComputeRayHits]);
        let pipeline = builder.build("Raycast Compute Pipeline");
        compute_pipelines.insert(ComputePipelineType::Raycast, pipeline);
        compute_pipelines
    }

    pub fn update_camera(&mut self, cam_x: f32, cam_y: f32, cam_angle: f32) {
        let dir_x = cam_angle.cos();
        let dir_y = cam_angle.sin();

        let fov_scale = 0.66;
        let plane_x = -dir_y * fov_scale;
        let plane_y = dir_x * fov_scale;

        let camera_data:[f32; 8] =[
            cam_x, cam_y,
            dir_x, dir_y,
            plane_x, plane_y,
            RENDER_WIDTH as f32, RENDER_HEIGHT as f32,
        ];

        self.queue.write_buffer(
            &self.camera_resources.buffer,
            0,
            bytemuck::cast_slice(&camera_data),
        );
    }

    pub fn update_map(&mut self, map_data: &[u32]) {
        self.queue.write_buffer(
        &self.map_resources.data_buffer,
        0,
        bytemuck::cast_slice(map_data),
    );
    }

    pub fn update_map_settings(&mut self, width: u32, height: u32, tile_size: u32, render_distance: u32) {
        let mut map_settings_data: [u32; 4] = [0; 4];
        map_settings_data[0] = width;
        map_settings_data[1] = height;
        map_settings_data[2] = tile_size;
        map_settings_data[3] = render_distance;

        self.queue.write_buffer(
            &self.map_resources.settings_buffer,
            0,
            bytemuck::cast_slice(&map_settings_data),
        );
    }

    pub fn update_surface(&mut self) {
        self.surface = self.instance.create_surface(self.window.clone()).unwrap();
        self.surface.configure(&self.device, &self.config);
    }
}