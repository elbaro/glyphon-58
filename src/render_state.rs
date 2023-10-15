use anyhow::Result;
use wgpu::TextureView;
use winit::dpi::PhysicalSize;

use crate::renderer::text::TextRenderer;

#[derive(Clone, Copy, Debug)]
pub struct Size {
    width: u32,
    height: u32,
}

impl From<&PhysicalSize<u32>> for Size {
    fn from(value: &PhysicalSize<u32>) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

pub struct RenderState {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    detail: RenderStateDetail,
    size: Size,
    text_renderer: TextRenderer,
}

enum RenderStateDetail {
    Window {
        surface: wgpu::Surface,
        surface_config: wgpu::SurfaceConfiguration,
    },
}

impl RenderState {
    pub fn size(&self) -> Size {
        self.size
    }
    pub async fn for_window(window: &winit::window::Window) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(&window) }?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let mut limits = wgpu::Limits::default();
        limits.max_texture_dimension_2d = 8192 * 4;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits,
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0], // vsync, fifo, etc
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let text_renderer = TextRenderer::new();
        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            size: Size {
                width: size.width,
                height: size.height,
            },
            detail: RenderStateDetail::Window {
                surface,
                surface_config,
            },
            text_renderer,
        })
    }

    pub fn resize(&mut self, size: Size) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            match &mut self.detail {
                RenderStateDetail::Window {
                    surface,
                    surface_config,
                } => {
                    surface_config.width = size.width;
                    surface_config.height = size.height;
                    surface.configure(&self.device, &surface_config);
                }
            }
        }
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        match &self.detail {
            RenderStateDetail::Window { surface, .. } => {
                let output = surface.get_current_texture()?;
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                self.draw(&view);
                output.present();
            }
        }
        Ok(())
    }

    fn draw(&self, view: &TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        self.text_renderer
            .render(&self.device, &self.queue, view, &mut encoder);
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
