mod resource;
use bevy_ecs::{
    schedule::IntoSystemConfigs as _,
    system::{NonSend, Res, ResMut, Resource},
};
pub use resource::Config;
use voxel_engine::{application::Application, ecs::{packages::{debug_gui::{self, DebugCompositor}, Package}, schedules::{Exit, Render}, systems}, utils::file_system};

/// Package for `Config`.
pub struct ConfigPackage;

impl Package for ConfigPackage {
    fn initialize(&mut self, app: &mut Application) {
        let config = file_system::read_config()
            .map(|cfg| {
                serde_yml::from_str::<Config>(&cfg)
                    .map(Some)
                    .unwrap_or_else(|e| {
                        log::error!("Could not deserialize config: {e}");
                        None
                    })
            })
            .unwrap_or_else(|e| {
                log::error!("Could not read config: {e}");
                None
            })
            .unwrap_or_default();

        app.insert_resource(config);
        app.insert_resource(ConfigDebugGuiState::default());
        app.add_systems(Exit, save_config_system);
        app.add_systems(
            Render,
            (config_debug_gui
                .after(debug_gui::start_gui_frame)
                .before(systems::render_system),),
        )
    }
}

/// Saves the `Config` resource.
fn save_config(config: &Config) {
    match serde_yml::to_string(config) {
        Ok(str) => {
            if let Err(e) = file_system::write_config(&str) {
                log::error!("Could not write config: {e}");
            }
        }
        Err(e) => log::error!("Could not serialize config: {e}"),
    };
}

/// The system that saves the `Config` resource.
fn save_config_system(config: Res<Config>) {
    save_config(&config);
}

/// Builds the ui for adjusting the `Config` at run time.
fn config_debug_gui(
    debug_compositor: Option<NonSend<DebugCompositor>>,
    mut config: ResMut<Config>,
    mut state: ResMut<ConfigDebugGuiState>,
) {
    if let Some(debug_compositor) = debug_compositor {
        let ui = debug_compositor.get_frame_ui();

        ui.main_menu_bar(|| {
            ui.menu("Windows", || {
                if ui.menu_item("Config") {
                    state.open = true;
                }
            })
        });

        if state.open {
            let mut open = state.open;
            ui.window("Config").opened(&mut open).build(|| {
                if ui.slider("Sensitivity", 0.0001, 1.0, &mut config.sensitivity) {
                    save_config(&config);
                }
                if ui.slider(
                    "Camera speed step",
                    0.1,
                    100.0,
                    &mut config.camera_speed_change_step,
                ) {
                    save_config(&config);
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text(
                        "How much the camera speed changes when scrolling the mouse wheel.",
                    );
                }
            });
            state.open = open;
        }
    }
}

/// Singleton state for the config window.
#[derive(Resource, Default)]
struct ConfigDebugGuiState {
    open: bool,
}
