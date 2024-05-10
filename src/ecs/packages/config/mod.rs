mod resource;
use bevy_ecs::system::Res;
pub use resource::Config;

use crate::{ecs::schedules::Exit, utils::file_system};

use super::Package;

/// Package for `Config`.
pub struct ConfigPackage;

impl Package for ConfigPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
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
        app.add_systems(Exit, save_config_system);
    }
}

/// Saves the `Config` resource.
fn save_config_system(config: Res<Config>) {
    match serde_yml::to_string(config.as_ref()) {
        Ok(str) => {
            if let Err(e) = file_system::write_config(&str) {
                log::error!("Could not write config: {e}");
            }
        }
        Err(e) => log::error!("Could not serialize config: {e}"),
    };
}
