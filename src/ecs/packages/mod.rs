use crate::application::Application;

pub mod camera_controller;
pub mod chunk;
pub mod config;
pub mod debug_gui;
pub mod generator;
pub mod input_provider;
pub mod pipeline_server;
pub mod render_init;
pub mod time;
pub mod voxel_registry;
pub mod window_surface;

/// The initialization stage of a package.
pub enum InitializationStage {
    /// The package will get initialized immediately.
    Init,
    /// The package will be initialized after the `WindowInit` schedule is run.
    WindowInit,
}

/// Trait for all packages.
pub trait Package {
    /// Initializes the package.
    fn initialize(&mut self, app: &mut Application);

    /// Specifies when the package should be initialized.
    fn intialization_stage(&self) -> InitializationStage {
        InitializationStage::Init
    }
}
