use winit::event_loop::{ControlFlow, EventLoop};

use super::Application;

/// Runs the given application with a `winit::event_loop::EventLoop`.
pub fn run_app(mut app: Application) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run_app(&mut app)?;

    Ok(())
}
