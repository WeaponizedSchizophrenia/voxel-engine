mod resource;
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::ResMut,
};
pub use resource::InputProvider;
use winit::{event::ElementState, keyboard::PhysicalKey};

use crate::ecs::{
    events::window_events::{KeyboardInput, MouseButtonInput, MouseMotion, MouseMoved},
    schedules::Update,
};

use super::Package;

pub struct InputProviderPackage;

impl Package for InputProviderPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        app.insert_resource(InputProvider::default());
        app.add_systems(
            Update,
            (
                mouse_motion_listener_system,
                keyboard_listener_system,
                mouse_button_listener_system,
            ),
        );
    }
}

pub fn mouse_motion_listener_system(
    mut events: EventReader<MouseMoved>,
    mut event_writer: EventWriter<MouseMotion>,
    mut input_provider: ResMut<InputProvider>,
) {
    for event in events.read() {
        let delta = input_provider.last_mouse_pos - event.new_position;
        event_writer.send(MouseMotion { delta });

        input_provider.last_mouse_pos = event.new_position;
    }
}

pub fn keyboard_listener_system(
    mut events: EventReader<KeyboardInput>,
    mut input_provider: ResMut<InputProvider>,
) {
    for event in events.read() {
        if let PhysicalKey::Code(key) = event.key {
            match event.state {
                ElementState::Pressed => input_provider.pressed_keys.set(key as usize, true),
                ElementState::Released => input_provider.pressed_keys.set(key as usize, false),
            }
        }
    }
}

pub fn mouse_button_listener_system(
    mut events: EventReader<MouseButtonInput>,
    mut input_provider: ResMut<InputProvider>,
) {
    for event in events.read() {
        match event.state {
            ElementState::Pressed => input_provider
                .pressed_mouse_buttons
                .set(resource::mouse_button_to_index(event.button), true),
            ElementState::Released => input_provider
                .pressed_mouse_buttons
                .set(resource::mouse_button_to_index(event.button), false),
        }
    }
}
