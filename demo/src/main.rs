use voxel_engine::{
    application::Application,
    ecs::packages::{
        camera_controller::CameraControllerPackage, chunk::ChunkPackage, config::ConfigPackage,
        debug_gui::DebugCompositorPackage, game_world::GameWorldPackage, gbuffer::GBufferPackage,
        generator::GeneratorPackage, input_provider::InputProviderPackage,
        logging_init::LoggingInitPackage, pipeline_server::PipelineServerPackage,
        voxel_registry::VoxelRegistryPackage,
    },
    ControlFlow, EventLoop,
};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = Application::new()?
        .with_package(LoggingInitPackage::with_custom_config("./config/log.yaml"))
        .with_package(ConfigPackage)
        .with_package(PipelineServerPackage)
        .with_package(GBufferPackage)
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