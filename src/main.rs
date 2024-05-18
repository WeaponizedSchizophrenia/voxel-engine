use application::Application;
use ecs::packages::{
    camera_controller::CameraControllerPackage, chunk::ChunkPackage, config::ConfigPackage, debug_gui::DebugCompositorPackage, game_world::GameWorldPackage, generator::GeneratorPackage, input_provider::InputProviderPackage, pipeline_server::PipelineServerPackage, voxel_registry::VoxelRegistryPackage
};
use winit::event_loop::{ControlFlow, EventLoop};

mod application;
mod common;
mod ecs;
mod rendering;
mod utils;

fn main() -> anyhow::Result<()> {
    match init_logging() {
        Ok(_) => log::info!("Logging intialized"),
        Err(e) => {
            eprintln!("Failed to initialize logging: {e}");
            println!("The application will continue without logging.");
        }
    }

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = Application::new()?
        .with_package(ConfigPackage)
        .with_package(PipelineServerPackage)
        .with_package(InputProviderPackage)
        .with_package(CameraControllerPackage)
        .with_package(GeneratorPackage)
        .with_package(VoxelRegistryPackage)
        .with_package(GameWorldPackage)
        .with_package(ChunkPackage)
        .with_package(DebugCompositorPackage);
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn init_logging() -> Result<(), anyhow::Error> {
    log4rs::init_file("./config/log.yaml", Default::default())
}
