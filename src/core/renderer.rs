use std::sync::Arc;
use winit::window::{Window};
use crate::core::pipeline_builder::PipelineBuilder;

pub struct WgpuState {
    instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    window: Arc<Window>,
    pub render_pipeline: wgpu::RenderPipeline,
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

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size: (size.width, size.height),
            window,
            render_pipeline,
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
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);
        drop(render_pass);

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.size = (new_width, new_height);
        self.config.width = new_width;
        self.config.height = new_height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn update_surface(&mut self) {
        self.surface = self.instance.create_surface(self.window.clone()).unwrap();
    }
}