use glyphon::{FontSystem, SwashCache};

pub struct TextRenderer {}

impl TextRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let color_attachments = wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                // load: wgpu::LoadOp::Load,
                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                store: true,
            },
        };
        let mut atlas = glyphon::TextAtlas::new(device, queue, wgpu::TextureFormat::Bgra8UnormSrgb);
        let mut text_renderer = glyphon::TextRenderer::new(
            &mut atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass: Text"),
            color_attachments: &[Some(color_attachments)],
            depth_stencil_attachment: None,
        });
        let mut font_system = FontSystem::new();
        let mut cache = SwashCache::new();
        {
            let mut buffer = glyphon::Buffer::new(
                &mut font_system,
                glyphon::Metrics::new(30.0 * 10.0, 42.0 * 10.0),
            );
            buffer.set_size(&mut font_system, 800.0, 600.0);
            buffer.set_text(
                &mut font_system,
                " 🦅 glyphon 🦁\nThe text x y z",
                glyphon::Attrs::new().family(glyphon::Family::SansSerif),
                glyphon::Shaping::Advanced,
            );
            buffer.shape_until_scroll(&mut font_system);
            text_renderer
                .prepare(
                    device,
                    queue,
                    &mut font_system,
                    &mut atlas,
                    glyphon::Resolution {
                        width: 800,
                        height: 600,
                    },
                    [glyphon::TextArea {
                        buffer: &buffer,
                        left: 100.0,
                        top: 100.0,
                        scale: 1.0,
                        bounds: glyphon::TextBounds {
                            left: 100,
                            top: 100,
                            right: 600,
                            bottom: 600,
                        },
                        default_color: glyphon::Color::rgb(0, 0, 0),
                    }],
                    &mut cache,
                )
                .unwrap();
            text_renderer.render(&atlas, &mut render_pass).unwrap();
        }

        drop(render_pass);
    }
}
