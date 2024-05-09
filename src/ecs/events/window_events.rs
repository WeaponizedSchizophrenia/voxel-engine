use bevy_ecs::event::Event;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent},
    keyboard::PhysicalKey,
};

/// Window resized event, contains the new width and height.
#[derive(Event, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct WindowResized {
    pub new_width: u32,
    pub new_height: u32,
}

impl From<PhysicalSize<u32>> for WindowResized {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self {
            new_width: value.width,
            new_height: value.height,
        }
    }
}

impl WindowResized {
    pub fn as_tuple(&self) -> (u32, u32) {
        (self.new_width, self.new_height)
    }
}

/// Window re-render request event.
#[derive(Event, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct WindowRenderRequested;

/// Keyboard input recieved for the window.
#[derive(Event, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct KeyboardInput {
    pub state: ElementState,
    pub key: PhysicalKey,
}

impl From<KeyEvent> for KeyboardInput {
    fn from(value: KeyEvent) -> Self {
        Self {
            state: value.state,
            key: value.physical_key,
        }
    }
}
