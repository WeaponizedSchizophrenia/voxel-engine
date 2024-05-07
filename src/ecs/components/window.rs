#![allow(dead_code)]

use std::sync::Arc;

use bevy_ecs::component::Component;
use winit::window::Window as WinitWindow;

#[derive(Component)]
pub struct MainWindow;

/// A window component.
#[derive(Component)]
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
