use crate::{ecs::{schedules::Render, systems}, rendering::pipelines::Pipeline};

use super::{debug_gui::{self, DebugCompositor}, pipeline_server::PipelineServer, render_init::RenderContext, Package};

mod resource;
use bevy_ecs::{schedule::IntoSystemConfigs, system::{NonSend, Res, ResMut, Resource}};
pub use resource::GameWorld;

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
        let voxel_pipeline = match pipeline_server.get_pipeline("voxel").map(AsRef::as_ref) {
            Some(Pipeline::Voxel(voxel_pipeline)) => voxel_pipeline,
            _ => {
                log::error!("Failed to get voxel pipeline");
                return;
            }
        };

        let game_world = GameWorld::new(&render_context.device, &voxel_pipeline.world_bind_group_layout);

        app.insert_resource(game_world);
        app.insert_resource(GameWorldDebuGuiState::default());
        app.add_systems(Render, 
            game_world_debug_gui.after(debug_gui::start_gui_frame)
                .before(systems::render_system)
        );
    }
}

#[derive(Resource, Default)]
struct GameWorldDebuGuiState {
    open: bool
}

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
            ui.window("Game World")
                .opened(&mut open)
                .build(|| {
                    if ui.slider_config(
                        "Sun direction",
                        -1.0,
                        1.0
                    ).build_array(game_world.sun_direction.as_mut_slice()) {
                        game_world.sun_direction = game_world.sun_direction.normalize();
                        game_world.update_uniform(&render_context.queue);
                    }
                    if ui.slider(
                        "Ambient light ammount",
                        0.0,
                        1.0,
                        &mut game_world.ambient_light
                    ) {
                        game_world.update_uniform(&render_context.queue);
                    }
                });
            state.open = open;
        }

    }
}
