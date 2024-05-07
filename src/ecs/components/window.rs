#![allow(dead_code)]

use std::sync::Arc;

use bevy_ecs::component::Component;
use winit::window::Window as WinitWindow;

#[derive(Component)]
pub struct Window(Arc<WinitWindow>);

impl Window {
    pub fn new(window: WinitWindow) -> Self {
        Self(window.into())
    }

    pub fn get_ptr(&self) -> Arc<WinitWindow> {
        self.0.clone()
    }
}
