use bevy_ecs::system::{Commands, Res};

use crate::{ecs::resources::Config, utils::file_system};

/// Initializes the `Config` resource.
pub fn init_config_system(mut commands: Commands) {
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

    commands.insert_resource(config);
}

pub fn save_config_system(config: Res<Config>) {
    match serde_yml::to_string(config.as_ref()) {
        Ok(str) => {
            if let Err(e) = file_system::write_config(&str) {
                log::error!("Could not write config: {e}");
            }
        }
        Err(e) => log::error!("Could not serialize config: {e}"),
    };
}
