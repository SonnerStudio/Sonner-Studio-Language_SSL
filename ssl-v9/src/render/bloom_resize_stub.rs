
    pub fn resize(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        // Re-create textures
        let (bright_texture, bright_view) = crate::render::bloom::BloomPass::create_texture(
            device, 
            config.width, 
            config.height, 
            wgpu::TextureFormat::Rgba16Float, 
            "Bloom Bright"
        );
        let (blur_temp_texture, blur_temp_view) = crate::render::bloom::BloomPass::create_texture(
            device, 
            config.width / 2, 
            config.height / 2, 
            wgpu::TextureFormat::Rgba16Float, 
            "Bloom Blur Temp"
        );
        let (blur_final_texture, blur_final_view) = crate::render::bloom::BloomPass::create_texture(
            device, 
            config.width / 2, 
            config.height / 2, 
            wgpu::TextureFormat::Rgba16Float, 
            "Bloom Blur Final"
        );
        
        self.bright_texture = bright_texture;
        self.bright_view = bright_view;
        self.blur_temp_texture = blur_temp_texture;
        self.blur_temp_view = blur_temp_view;
        self.blur_final_texture = blur_final_texture;
        self.blur_final_view = blur_final_view;
        
        // Re-create bind groups (they depend on views)
        let bright_pass_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bloom Bright Pass Bind Group"),
            layout: &self.bright_pass_pipeline.get_bind_group_layout(0), // Assuming layout 0 matches but we stored layouts?
            // Actually usually we store layouts. Let's look at struct.
            // BloomPass likely doesn't store layouts publicly? 
            // If we don't store layouts, we might struggle to recreate bind groups without them.
            // Let's check struct definition again.
            entries: &[
                 // ... this is risky without layouts
            ],
        });
        // Strategy B: If BloomPass doesn't store layouts, we can't easily resize without refactoring.
        // Let's check struct.
    }
