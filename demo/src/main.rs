use camera_controller::CameraControllerPackage;
use voxel_engine::application::Application;

mod camera_controller;

fn main() -> anyhow::Result<()> {
    Application::new()?
        .with_package(
            voxel_engine::ecs::packages::logging_init::LoggingInitPackage::with_custom_config(
                "./config/log.yml",
            ),
        )
        .with_package(voxel_engine::ecs::packages::config::ConfigPackage)
        .with_package(voxel_engine::ecs::packages::pipeline_server::PipelineServerPackage)
        .with_package(voxel_engine::ecs::packages::gbuffer::GBufferPackage)
        .with_package(voxel_engine::ecs::packages::input_provider::InputProviderPackage)
        .with_package(voxel_engine::ecs::packages::generator::GeneratorPackage)
        .with_package(voxel_engine::ecs::packages::voxel_registry::VoxelRegistryPackage)
        .with_package(voxel_engine::ecs::packages::game_world::GameWorldPackage)
        .with_package(voxel_engine::ecs::packages::chunk::ChunkPackage)
        .with_package(voxel_engine::ecs::packages::debug_gui::DebugCompositorPackage)
        .with_package(CameraControllerPackage)
        .run()
}
