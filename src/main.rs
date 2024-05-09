use application::Application;
use ecs::packages::{
    camera_controller::CameraControllerPackage, config::ConfigPackage,
    input_provider::InputProviderPackage, pipeline_server::PipelineServerPackage,
};
use log4rs::config::Deserializers;
use winit::event_loop::{ControlFlow, EventLoop};

mod application;
mod common;
mod ecs;
mod rendering;
mod utils;

#[pollster::main]
async fn main() -> anyhow::Result<()> {
    match init_logging() {
        Ok(_) => log::info!("Logging intialized"),
        Err(e) => {
            eprintln!("Failed to initialize logging: {e}");
            println!("The application will continue without logging.");
        }
    }

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = Application::new()
        .await?
        .with_package(ConfigPackage)
        .with_package(PipelineServerPackage)
        .with_package(InputProviderPackage)
        .with_package(CameraControllerPackage);
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn init_logging() -> Result<(), anyhow::Error> {
    log4rs::init_file("./config/log.yaml", Deserializers::default())
}
