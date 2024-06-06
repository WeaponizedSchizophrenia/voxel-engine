use crate::{
    ecs::{schedules::Render, systems},
    rendering::pipelines::Pipeline,
};

use super::{
    debug_gui::{self, DebugCompositor},
    pipeline_server::PipelineServer,
    render_init::RenderContext,
    Package,
};

mod resource;
use bevy_ecs::{
    schedule::IntoSystemConfigs,
    system::{NonSend, Res, ResMut, Resource},
};
use nalgebra::UnitVector3;
pub use resource::GameWorld;

/// Package for the `GameWorld` resource.
pub struct GameWorldPackage;

impl Package for GameWorldPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let render_context = match app.get_resource::<RenderContext>() {
            Some(render_context) => render_context,
            None => {
                log::error!("Failed to get render context");
                return;
            }
        };
        let pipeline_server = match app.get_resource::<PipelineServer>() {
            Some(pipeline_server) => pipeline_server,
            None => {
                log::error!("Failed to get pipeline server");
                return;
            }
        };
        let lighting_pipeline = match pipeline_server.get_pipeline("lighting") {
            Some(Pipeline::Lighting(pipeline)) => pipeline,
            _ => {
                log::error!("Failed to get lighting pipeline");
                return;
            }
        };

        let game_world = GameWorld::new(
            &render_context.device,
            &lighting_pipeline.world_bind_group_layout,
        );

        app.insert_resource(game_world);
        app.insert_resource(GameWorldDebuGuiState::default());
        app.add_systems(
            Render,
            game_world_debug_gui
                .after(debug_gui::start_gui_frame)
                .before(systems::render_system),
        );
    }
}

/// Builds a ui for the game world.
fn game_world_debug_gui(
    debug_compositor: Option<NonSend<DebugCompositor>>,
    render_context: Res<RenderContext>,
    mut state: ResMut<GameWorldDebuGuiState>,
    mut game_world: ResMut<GameWorld>,
) {
    if let Some(debug_compositor) = debug_compositor {
        let ui = debug_compositor.get_frame_ui();

        ui.main_menu_bar(|| {
            ui.menu("Windows", || {
                if ui.menu_item("Game World") {
                    state.open = true;
                }
            })
        });

        if state.open {
            let mut open = state.open;
            ui.window("Game World").opened(&mut open).build(|| {
                let mut sun_direction = game_world.sun_direction.into_inner();
                if ui
                    .slider_config("Sun direction", -1.0, 1.0)
                    .build_array(sun_direction.as_mut_slice())
                {
                    game_world.sun_direction = UnitVector3::new_normalize(sun_direction);
                    game_world.update_uniform(&render_context.queue);
                }
                if ui.slider(
                    "Ambient light ammount",
                    0.0,
                    1.0,
                    &mut game_world.ambient_light,
                ) {
                    game_world.update_uniform(&render_context.queue);
                }
            });
            state.open = open;
        }
    }
}

/// Singleton state for the game world window.
#[derive(Resource, Default)]
struct GameWorldDebuGuiState {
    open: bool,
}
