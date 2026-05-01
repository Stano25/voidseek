use std::sync::Arc;
use winit::window::{Window};
use crate::core::pipeline;
use wgpu::util::DeviceExt;
use std::collections::HashMap;
use crate::core::definitions::{PipelineType, BindScope};
use crate::core::bind_group_layout;

pub struct WgpuState {
    instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    window: Arc<Window>,
    render_pipelines: HashMap<PipelineType, wgpu::RenderPipeline>,
    bind_group_layouts: HashMap<BindScope, wgpu::BindGroupLayout>,

    
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    map_bind_group: wgpu::BindGroup,
    map_buffer: wgpu::Buffer,
    
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

        let divice_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&divice_descriptor).await.unwrap();

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

        let bind_group_layouts = Self::build_bind_groups_layouts(&device);

        let render_pipelines = Self::build_pipelines(&device, &config, &bind_group_layouts);
        
        
        let camera_data:[f32; 8] =[0.0, 0.0, 1.0, 0.0, 0.0, 0.66, 800.0, 600.0]; 
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

        let map_data: Vec<u32> = vec![
            8,
            8,
            64,
            0,
            1,1,1,1,1,1,1,1,
            1,0,1,0,0,0,0,1,
            1,0,1,0,0,0,0,1,
            1,0,1,0,0,0,0,1,
            1,0,0,0,0,0,0,1,
            1,0,0,0,0,1,0,1,
            1,0,0,0,0,0,0,1,
            1,1,1,1,1,1,1,1,
        ];
        
        let map_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map Buffer"),
            contents: bytemuck::cast_slice(&map_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let map_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Map Bind Group"),
            layout: &bind_group_layouts[&BindScope::Map],
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: map_buffer.as_entire_binding(),
            }],
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
            camera_bind_group,
            camera_buffer,
            map_bind_group,
            map_buffer,
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

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 0.0 }),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        };

        let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
        let render_pipeline = self.render_pipelines.get(&PipelineType::Raycast).unwrap();
        render_pass.set_pipeline(render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.map_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
        drop(render_pass);

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

        layout = builder.build("Map Bind Group Layout");
        layouts.insert(BindScope::Map, layout);

        layouts
    }

    fn build_pipelines(device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &HashMap<BindScope, wgpu::BindGroupLayout>)
        -> HashMap<PipelineType, wgpu::RenderPipeline> {

        let mut pipelines = HashMap::new();
        let mut builder = pipeline::Builder::new(device);
        let mut pipeline: wgpu::RenderPipeline;
        builder.set_shader_module("raycast.wgsl", "vs_main", "fs_main");
        builder.set_pixel_format(config.format);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::Camera]);
        builder.add_bind_group_layout(&bind_group_layouts[&BindScope::Map]);
        pipeline = builder.build("Raycast Pipeline");
        
        pipelines.insert(PipelineType::Raycast, pipeline);

        pipelines
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
            self.config.width as f32, self.config.height as f32,
        ];

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&camera_data),
        );
    }

    pub fn update_map(&mut self, map_data: &[u32]) {

        self.queue.write_buffer(
            &self.map_buffer,
            0,
            bytemuck::cast_slice(map_data),
        );
    }

    pub fn update_surface(&mut self) {
        self.surface = self.instance.create_surface(self.window.clone()).unwrap();
    }
}