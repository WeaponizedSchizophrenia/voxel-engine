use crate::{
    application::Application,
    ecs::{
        events::window_events::WindowEvent,
        schedules::{Render, Update},
        systems,
    },
};

use super::{
    render_init::RenderContext, time::Time, window_surface::Window, InitializationStage, Package,
};

mod resource;
use bevy_ecs::{
    event::EventReader,
    schedule::IntoSystemConfigs as _,
    system::{NonSendMut, Res},
};
pub use resource::DebugCompositor;

/// Package for `DebugCompositor`.
pub struct DebugCompositorPackage;

impl Package for DebugCompositorPackage {
    fn initialize(&mut self, app: &mut Application) {
        let window = match app.get_resource::<Window>() {
            Some(window) => window,
            None => {
                log::error!("Failed to get window");
                return;
            }
        };
        let render_context = match app.get_resource::<RenderContext>() {
            Some(rc) => rc,
            None => {
                log::error!("Failed to get render context");
                return;
            }
        };

        app.insert_non_send_resource(DebugCompositor::new(&window, &render_context));
        app.add_systems(Update, update_gui);
        app.add_systems(Render, start_gui_frame.before(systems::render_system));
    }

    fn intialization_stage(&self) -> InitializationStage {
        InitializationStage::WindowInit
    }
}

/// Updates the gui
pub fn update_gui(
    mut window_events: EventReader<WindowEvent>,
    mut debug_compositor: NonSendMut<DebugCompositor>,
    time: Res<Time>,
    window: Res<Window>,
) {
    debug_compositor.update_delta_time(time.get_delta_time());

    for event in window_events.read() {
        debug_compositor.handle_event(&window, event);
    }
}

/// Starts the frame that the gui will be renderered to.
pub fn start_gui_frame(mut debug_compositor: NonSendMut<DebugCompositor>) {
    debug_compositor.start_frame();
}
