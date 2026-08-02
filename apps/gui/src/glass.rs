use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use winit::window::Window;

// WAIT

struct Bell {
    rung: Mutex<bool>,
    wake: Condvar,
}

impl Wake for Bell {
    fn wake(self: Arc<Self>) {
        *self.rung.lock().unwrap() = true;
        self.wake.notify_one();
    }
}

fn settle<F: Future>(task: F) -> F::Output {
    let bell = Arc::new(Bell {
        rung: Mutex::new(false),
        wake: Condvar::new(),
    });
    let waker = Waker::from(bell.clone());
    let mut cx = Context::from_waker(&waker);
    let mut task = Box::pin(task);
    loop {
        if let Poll::Ready(out) = Pin::new(&mut task).poll(&mut cx) {
            return out;
        }
        let mut rung = bell.rung.lock().unwrap();
        while !*rung {
            rung = bell.wake.wait(rung).unwrap();
        }
        *rung = false;
    }
}

// GLASS

pub struct Glass {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    most: u32,
    pipe: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    flat: wgpu::Sampler,
    rect: wgpu::Buffer,
    sheet: Option<(wgpu::Texture, usize, usize)>,
}

impl Glass {
    pub fn new(window: Arc<Window>) -> Option<Glass> {
        let size = window.inner_size();
        let (w, h) = (size.width.max(1), size.height.max(1));
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).ok()?;
        let adapter = settle(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .ok()?;
        let (device, queue) = settle(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("mrly"),
            required_limits: adapter.limits(),
            ..Default::default()
        }))
        .ok()?;
        let most = device.limits().max_texture_dimension_2d;
        let mut config = surface.get_default_config(&adapter, w.min(most), h.min(most))?;
        config.usage = wgpu::TextureUsages::RENDER_ATTACHMENT;
        config.present_mode = wgpu::PresentMode::AutoVsync;
        let format = config.format;
        surface.configure(&device, &config);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sheet"),
            source: wgpu::ShaderSource::Wgsl(include_str!("sheet.wgsl").into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sheet"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let pipe_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sheet"),
            bind_group_layouts: &[Some(&layout)],
            ..Default::default()
        });
        let pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sheet"),
            layout: Some(&pipe_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(format.into())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let flat = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("flat"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let rect = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rect"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Some(Glass {
            surface,
            device,
            queue,
            config,
            most,
            pipe,
            layout,
            flat,
            rect,
            sheet: None,
        })
    }

    pub fn size(&self) -> (usize, usize) {
        (self.config.width as usize, self.config.height as usize)
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        let (w, h) = (w.min(self.most), h.min(self.most));
        if w == 0 || h == 0 || (w == self.config.width && h == self.config.height) {
            return;
        }
        self.config.width = w;
        self.config.height = h;
        self.surface.configure(&self.device, &self.config);
    }

    fn keep(&mut self, w: usize, h: usize) {
        let stale = !matches!(&self.sheet, Some((_, tw, th)) if *tw == w && *th == h);
        if stale {
            let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("sheet"),
                size: wgpu::Extent3d {
                    width: w as u32,
                    height: h as u32,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            self.sheet = Some((texture, w, h));
        }
    }

    pub fn present(
        &mut self,
        pixels: &[[u8; 4]],
        w: usize,
        h: usize,
        board: [u8; 4],
        scale: usize,
    ) {
        if w == 0 || h == 0 || pixels.len() < w * h {
            return;
        }
        let flat: Vec<u8> = pixels[..w * h].iter().flatten().copied().collect();
        self.keep(w, h);
        let texture = &self.sheet.as_ref().expect("a sheet").0;
        let extent = wgpu::Extent3d {
            width: w as u32,
            height: h as u32,
            depth_or_array_layers: 1,
        };
        self.queue.write_texture(
            texture.as_image_copy(),
            &flat,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * w as u32),
                rows_per_image: Some(h as u32),
            },
            extent,
        );
        let (bw, bh) = (self.config.width as f32, self.config.height as f32);
        let (sw, sh) = ((w * scale) as f32, (h * scale) as f32);
        let rect = [
            ((bw - sw) / 2.0).max(0.0) / bw,
            ((bh - sh) / 2.0).max(0.0) / bh,
            sw / bw,
            sh / bh,
        ];
        self.queue
            .write_buffer(&self.rect, 0, bytemuck(&rect).as_slice());
        let view = self
            .sheet
            .as_ref()
            .expect("a sheet")
            .0
            .create_view(&wgpu::TextureViewDescriptor::default());
        let bind = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sheet"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.flat),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.rect.as_entire_binding(),
                },
            ],
        });
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            _ => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };
        let target = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("sheet"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sheet"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(desk(board)),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipe);
            pass.set_bind_group(0, &bind, &[]);
            pass.draw(0..4, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        self.queue.present(frame);
    }
}

fn desk(board: [u8; 4]) -> wgpu::Color {
    let lift = |c: u8| {
        let v = c as f64 / 255.0;
        match v <= 0.04045 {
            true => v / 12.92,
            false => ((v + 0.055) / 1.055).powf(2.4),
        }
    };
    wgpu::Color {
        r: lift(board[0]),
        g: lift(board[1]),
        b: lift(board[2]),
        a: 1.0,
    }
}

fn bytemuck(rect: &[f32; 4]) -> Vec<u8> {
    rect.iter().flat_map(|f| f.to_ne_bytes()).collect()
}
