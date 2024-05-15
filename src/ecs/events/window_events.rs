use bevy_ecs::{event::Event, world::World};
use nalgebra::{vector, Vector2};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseButton},
    keyboard::PhysicalKey,
};

use crate::utils::bevy::WorldExtensions;

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
    /// Returns the new width and height as a tuple.
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
    /// The state of the input.
    pub state: ElementState,
    /// The button that was pressed or released.
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

/// Mouse moved in the window, provides the delta.
#[derive(Event, Clone, Copy, PartialEq, Debug)]
pub struct MouseMotion {
    /// The difference of the mouse position since the last frame.
    pub delta: Vector2<f32>,
}

/// Mouse moved in the window, provides the position.
#[derive(Event, Clone, Copy, PartialEq, Debug)]
pub struct MouseMoved {
    // The new position of the mouse.
    pub new_position: Vector2<f32>,
}

impl From<PhysicalPosition<f64>> for MouseMoved {
    fn from(value: PhysicalPosition<f64>) -> Self {
        Self {
            new_position: vector![value.x as f32, value.y as f32],
        }
    }
}

/// A mouse button input event has been recieved for the window.
#[derive(Event, Clone, Copy, PartialEq, Debug)]
pub struct MouseButtonInput {
    /// The state of the input.
    pub state: ElementState,
    /// The button that was pressed or released.
    pub button: MouseButton,
}

/// Registers all the window events to the provided `World`.
pub fn register_window_events(world: &mut World) {
    world.add_event::<WindowResized>();
    world.add_event::<WindowRenderRequested>();
    world.add_event::<KeyboardInput>();
    world.add_event::<MouseMotion>();
    world.add_event::<MouseMoved>();
    world.add_event::<MouseButtonInput>();
}
