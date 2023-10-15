mod render_state;
mod renderer;

use render_state::RenderState;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    let mut state = RenderState::for_window(&window).await?; // 800, 600
    state.render()?;

    event_loop.run(move |ev, _, control_flow| match ev {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size.into());
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize((&**new_inner_size).into());
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        _ => {}
    });

    Ok(())
}

fn main() {
    pollster::block_on(run()).unwrap();
}
