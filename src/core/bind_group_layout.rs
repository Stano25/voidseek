pub struct Builder<'a> {
    entries: Vec<(wgpu::BindGroupLayoutEntry)>,
    device: &'a wgpu::Device,
}

impl<'a> Builder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            entries: Vec::new(),
            device: device,
        }
    }

    fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn add_entry(&mut self, entry: wgpu::BindGroupLayoutEntry) {
        self.entries.push(entry);
    }

    pub fn build(&mut self, label: &str) -> wgpu::BindGroupLayout {
        let layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &self.entries,
        });

        self.reset();

        layout
    }
}