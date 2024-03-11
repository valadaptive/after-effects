use wgpu::*;

pub struct BufferState {
    pub in_size: (usize, usize, usize),
    pub out_size: (usize, usize, usize),
    pub in_texture: Texture,
    pub out_texture: Texture,
    pub pipeline: ComputePipeline,
    pub bind_group: BindGroup,
    // This staging buffer would have to be in a thread_local hashmap (with dimensions as a key)
    // pub staging_buffer: Buffer,
    // pub padded_out_stride: u32,
}

pub struct WgpuProcessing<T: Sized> {
    _adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub shader: ShaderModule,
    pub params: Buffer,
    pub state: Option<BufferState>,
    _marker: std::marker::PhantomData<T>,
}

#[allow(dead_code)]
pub enum ProcShaderSource<'a> {
    Wgsl(&'a str),
    SpirV(&'a [u8])
}

impl<T: Sized> WgpuProcessing<T> {
    pub fn new(shader: ProcShaderSource) -> Self {
        let power_preference = util::power_preference_from_env().unwrap_or(PowerPreference::HighPerformance);
        let instance = Instance::new(InstanceDescriptor::default());

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions { power_preference, ..Default::default() })).unwrap();

        let (device, queue) = pollster::block_on(
            adapter.request_device(&DeviceDescriptor { label: None, required_features: adapter.features(), required_limits: adapter.limits() }, None)
        ).unwrap();

        let info = adapter.get_info();
        log::info!("Using {} ({}) - {:#?}.", info.name, info.device, info.backend);

        let shader = device.create_shader_module(match shader {
            ProcShaderSource::SpirV(bytes) => {
                ShaderModuleDescriptor {
                    label: None,
                    source: util::make_spirv(&bytes),
                }
            },
            ProcShaderSource::Wgsl(wgsl) => {
                ShaderModuleDescriptor {
                    label: None,
                    source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(wgsl)),
                }
            }
        });

        let params = device.create_buffer(&BufferDescriptor { size: std::mem::size_of::<T>() as u64, usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST, label: None, mapped_at_creation: false });

        Self {
            _adapter: adapter,
            device,
            queue,
            shader,
            params,
            _marker: std::marker::PhantomData,
            state: None
        }
    }

    pub fn setup_size(&mut self, in_size: (usize, usize, usize), out_size: (usize, usize, usize)) {
        if let Some(ref state) = self.state {
            if state.in_size != in_size || state.out_size != out_size {
                self.state = Some(self.create_buffers(in_size, out_size));
            }
        } else {
            self.state = Some(self.create_buffers(in_size, out_size));
        }
    }

    pub fn create_buffers(&self, in_size: (usize, usize, usize), out_size: (usize, usize, usize)) -> BufferState {
        let (iw, ih, _)  = (in_size.0  as u32, in_size.1  as u32, in_size.2  as u32);
        let (ow, oh, _os) = (out_size.0 as u32, out_size.1 as u32, out_size.2 as u32);

        // let align = COPY_BYTES_PER_ROW_ALIGNMENT as u32;
        // let padding = (align - _os % align) % align;
        // let padded_out_stride = _os + padding;
        // let staging_size = padded_out_stride * oh;

        let in_desc = TextureDescriptor {
            label: None,
            size: Extent3d { width: iw, height: ih, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Uint,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST,
            view_formats: &[]
        };
        let out_desc = TextureDescriptor {
            label: None,
            size: Extent3d { width: ow, height: oh, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Uint,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC | TextureUsages::COPY_DST,
            view_formats: &[]
        };

        let in_texture = self.device.create_texture(&in_desc);
        let out_texture = self.device.create_texture(&out_desc);
        // let staging_buffer = self.device.create_buffer(&BufferDescriptor { size: staging_size as u64, usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST, label: None, mapped_at_creation: false });

        let in_view = in_texture.create_view(&TextureViewDescriptor::default());
        let out_view = out_texture.create_view(&TextureViewDescriptor::default());

        let layout = self.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry { binding: 0, visibility: ShaderStages::COMPUTE, ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: BufferSize::new(std::mem::size_of::<T>() as _) }, count: None },
                BindGroupLayoutEntry { binding: 1, visibility: ShaderStages::COMPUTE, ty: BindingType::Texture { sample_type: TextureSampleType::Uint, view_dimension: TextureViewDimension::D2, multisampled: false }, count: None },
                BindGroupLayoutEntry { binding: 2, visibility: ShaderStages::COMPUTE, ty: BindingType::StorageTexture { access: StorageTextureAccess::ReadWrite, format: TextureFormat::Rgba8Uint, view_dimension: TextureViewDimension::D2 }, count: None },
            ],
            label: None,
        });

        let pipeline_layout = self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });

        let pipeline = self.device.create_compute_pipeline(&ComputePipelineDescriptor {
            module: &self.shader,
            entry_point: "main",
            label: None,
            layout: Some(&pipeline_layout),
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry { binding: 0, resource: self.params.as_entire_binding() },
                BindGroupEntry { binding: 1, resource: BindingResource::TextureView(&in_view) },
                BindGroupEntry { binding: 2, resource: BindingResource::TextureView(&out_view) },
            ],
        });

        BufferState {
            in_size,
            out_size,
            in_texture,
            out_texture,
            pipeline,
            bind_group,
            // staging_buffer,
            // padded_out_stride
        }
    }

    pub fn run_compute(&self, params: &T, in_size: (usize, usize, usize), out_size: (usize, usize, usize), in_buffer: &[u8], out_buffer: &mut [u8]) -> bool {
        let state = self.state.as_ref().unwrap();

        let width = out_size.0 as u32;
        let height = out_size.1 as u32;

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: None });

        // Write params uniform
        self.queue.write_buffer(
            &self.params,
            0,
            unsafe { std::slice::from_raw_parts(params as *const _ as _, std::mem::size_of::<T>() ) }
        );

        // Write input texture
        self.queue.write_texture(
            state.in_texture.as_image_copy(),
            in_buffer,
            ImageDataLayout { offset: 0, bytes_per_row: Some(in_size.2 as u32), rows_per_image: None },
            Extent3d { width: in_size.0 as u32, height: in_size.1 as u32, depth_or_array_layers: 1 },
        );

        // Run the compute pass
        {
            let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor { label: None, timestamp_writes: None });
            cpass.set_pipeline(&state.pipeline);
            cpass.set_bind_group(0, &state.bind_group, &[]);
            cpass.dispatch_workgroups((width as f32 / 16.0).ceil() as u32, (height as f32 / 16.0).ceil() as u32, 1);
        }

        // Create staging buffer
        let align = COPY_BYTES_PER_ROW_ALIGNMENT as u32;
        let padding = (align - out_size.2 as u32 % align) % align;
        let padded_out_stride = out_size.2 as u32 + padding;
        let staging_size = padded_out_stride * out_size.1 as u32;
        let staging_buffer = self.device.create_buffer(&BufferDescriptor { size: staging_size as u64, usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST, label: None, mapped_at_creation: false });

        // Copy output texture to buffer that we can read
        encoder.copy_texture_to_buffer(
            ImageCopyTexture { texture: &state.out_texture, mip_level: 0, origin: Origin3d::ZERO, aspect: TextureAspect::All },
            ImageCopyBuffer { buffer: &staging_buffer, layout: ImageDataLayout { offset: 0, bytes_per_row: Some(padded_out_stride), rows_per_image: None } },
            Extent3d { width: width as u32, height: height as u32, depth_or_array_layers: 1 }
        );

        self.queue.submit(Some(encoder.finish()));

        // Read the output buffer
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(MapMode::Read, move |v| sender.send(v).unwrap());

        self.device.poll(Maintain::Wait);

        if let Some(Ok(())) = pollster::block_on(receiver.receive()) {
            let out_stride = out_size.2;

            let data = buffer_slice.get_mapped_range();
            if padded_out_stride == out_stride as u32 {
                // Fast path
                (&mut out_buffer[..height as usize * out_stride]).copy_from_slice(data.as_ref());
            } else {
                data.as_ref()
                    .chunks(padded_out_stride as usize)
                    .zip(out_buffer.chunks_mut(out_stride))
                    .for_each(|(src, dest)| {
                        dest.copy_from_slice(&src[0..out_stride]);
                    });
            }

            // We have to make sure all mapped views are dropped before we unmap the buffer.
            drop(data);
            staging_buffer.unmap();
        } else {
            log::error!("failed to run compute on wgpu!");
            return false;
        }
        true
    }
}
