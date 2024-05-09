use bevy_ecs::{
    event::EventReader,
    system::{Res, ResMut},
};

use crate::ecs::{
    events::{window_events::KeyboardInput, WindowRenderRequested, WindowResized},
    resources::{RenderContext, Window, WindowRenderSurface},
};

/// Requests a rerender for each window.
pub fn rerender_request_system(
    mut events: EventReader<WindowRenderRequested>,
    window: Option<Res<Window>>,
) {
    if let Some(window) = window {
        for _event in events.read() {
            window.request_rerender();
        }
    }
}

pub fn resized_system(
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

pub fn keyboard_input_system(mut events: EventReader<KeyboardInput>) {
    let _ = &mut events;
    // for _event in events.read() {}
}
