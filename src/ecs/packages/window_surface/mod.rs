mod window;
use std::mem;

use bevy_ecs::{event::EventReader, system::{Res, ResMut}};
pub use window::Window;
mod surface;
pub use surface::WindowRenderSurface;

use crate::ecs::{
    events::{WindowRenderRequested, WindowResized}, packages::render_init::{GpuInstance, RenderContext}, schedules::{SentWindowEvent, WindowInit}
};

use super::Package;

/// Package for initializing a window and it's surface.
pub struct WindowSurfacePackage {
    /// Even though this is a `Option`, it is always `Some`,
    /// because creating this struct from outside this module is only available with `WindowSurfacePackage::new()`.
    winit_window: Option<winit::window::Window>,
}

impl WindowSurfacePackage {
    /// Creates a new `WindowSurfacePackage` with the provided `winit_window`.
    pub fn new(winit_window: winit::window::Window) -> Self {
        Self {
            winit_window: Some(winit_window),
        }
    }
}

impl Package for WindowSurfacePackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let instance = match app.get_resource::<GpuInstance>() {
            Some(instance) => instance,
            None => {
                log::error!("Failed to get GpuInstance");
                return;
            }
        };
        let render_context = match app.get_resource::<RenderContext>() {
            Some(rc) => rc,
            None => {
                log::error!("Failed to get RenderContext");
                return;
            }
        };

        // This can be safely unwraped.
        let window = mem::replace(self, Self { winit_window: None })
            .winit_window
            .unwrap();

        let window = Window::new(window);
        let surface =
            match WindowRenderSurface::render_to_window(&window, &instance, &render_context) {
                Ok(surface) => surface,
                Err(e) => {
                    log::error!("Failed to create render surface: {e}");
                    return;
                }
            };

        app.insert_resource(window);
        app.insert_resource(surface);

        log::info!("Window and surface created");

        app.add_systems(SentWindowEvent, (rerender_request_system, resized_system));

        app.run_schedule(WindowInit);
    }
}

/// Requests a rerender for each window.
fn rerender_request_system(
    mut events: EventReader<WindowRenderRequested>,
    window: Option<Res<Window>>,
) {
    if let Some(window) = window {
        for _event in events.read() {
            window.request_rerender();
        }
    }
}

/// Resizes the surface if one exists.
fn resized_system(
    mut events: EventReader<WindowResized>,
    surface: Option<ResMut<WindowRenderSurface>>,
    render_context: Res<RenderContext>,
) {
    if let Some(mut surface) = surface {
        for event in events.read() {
            surface.resize(&render_context, event.as_tuple());
        }
    } else {
        log::warn!("No surface to resize.");
    }
}
