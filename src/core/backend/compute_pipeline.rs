use std::env::current_dir;
use std::fs;

pub struct Builder<'a> {
    shader_filename: String,
    entry: String,
    bind_group_layouts: Vec<Option<&'a wgpu::BindGroupLayout>>,
    device: &'a wgpu::Device,
}

impl<'a> Builder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            shader_filename: "".to_string(),
            entry: "".to_string(),
            bind_group_layouts: Vec::new(),
            device,
        }
    }

    pub fn set_shader_module(&mut self, shader_filename: &str, entry: &str) {
        self.shader_filename = shader_filename.to_string();
        self.entry = entry.to_string();
    }

    pub fn add_bind_group_layout(&mut self, layout: &'a wgpu::BindGroupLayout) {
        self.bind_group_layouts.push(Some(layout));
    }

    pub fn build(&mut self, label: &str) -> wgpu::ComputePipeline {
        let mut filepath = current_dir().unwrap();
        filepath.push("src");
        filepath.push("shaders");
        filepath.push(self.shader_filename.as_str());

        let source_code = fs::read_to_string(filepath).expect("Failed to read shader file");

        let shader_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        });

        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &self.bind_group_layouts,
            immediate_size: 0,
        });

        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(label),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: Some(self.entry.as_str()),
            compilation_options: Default::default(),
            cache: None,
        });

        self.bind_group_layouts.clear();
        pipeline
    }
}