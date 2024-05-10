use bevy_ecs::system::Resource;
use bitvec::{array::BitArray, BitArr};
use nalgebra::Vector2;
use winit::{event::MouseButton, keyboard::KeyCode};

/// Handles the input for the application and provides useful methods for systems.
#[derive(Resource, Debug)]
pub struct InputProvider {
    pub(super) last_mouse_pos: Vector2<f32>,
    pub(super) pressed_keys: BitArr!(for 256),
    pub(super) pressed_mouse_buttons: BitArr!(for 6),
}

impl Default for InputProvider {
    fn default() -> Self {
        Self {
            last_mouse_pos: Vector2::zeros(),
            pressed_keys: BitArray::ZERO,
            pressed_mouse_buttons: BitArray::ZERO,
        }
    }
}

impl InputProvider {
    /// Returns true if the key is pressed.
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        *self.pressed_keys.get(key as usize).unwrap()
    }

    /// Returns true if the mouse button is pressed.
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        *self
            .pressed_mouse_buttons
            .get(mouse_button_to_index(button))
            .unwrap()
    }
}

/// Encodes the mouse button as an index.
pub(super) fn mouse_button_to_index(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(_) => 5,
    }
}
