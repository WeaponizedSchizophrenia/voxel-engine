use bevy_ecs::event::Event;
use nalgebra::{vector, Vector2};
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

#[derive(Event, Clone, Copy, PartialEq, Debug)]
pub struct MouseMotion {
    pub new_position: Vector2<f32>,
}

impl MouseMotion {
    pub fn new<V2: Into<(f64, f64)>>(new_position: V2) -> Self {
        let new_position = new_position.into();
        Self {
            new_position: vector![new_position.0 as f32, new_position.1 as f32],
        }
    }
}