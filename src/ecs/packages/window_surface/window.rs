use std::sync::Arc;

use bevy_ecs::system::Resource;
use winit::window::Window as WinitWindow;

/// A window resource.
#[derive(Resource)]
pub struct Window(Arc<WinitWindow>);

impl From<WinitWindow> for Window {
    fn from(value: WinitWindow) -> Self {
        Self::new(value)
    }
}

impl Window {
    /// Creates a new `Window` from a winit window.
    pub fn new(window: WinitWindow) -> Self {
        Self(window.into())
    }

    /// Returns a shared pointer to the winit window.
    pub fn get_ptr(&self) -> Arc<WinitWindow> {
        self.0.clone()
    }

    /// Requests a rerender for this window.
    pub fn request_rerender(&self) {
        self.0.request_redraw();
    }

    /// Gets the aspect ratio of the window.
    pub fn get_aspect_ratio(&self) -> f32 {
        let size = self.0.inner_size();
        size.width as f32 / size.height as f32
    }
}
