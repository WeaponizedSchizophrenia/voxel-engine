use application::Application;
use log4rs::config::Deserializers;
use winit::{
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop},
};

mod application;

fn main() -> Result<(), EventLoopError> {
    match init_logging() {
        Ok(_) => log::info!("Logging intialized"),
        Err(e) => {
            eprintln!("Failed to initialize logging: {e}");
            println!("The application will continue without logging.");
        }
    }

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut application = Application::default();
    event_loop.run_app(&mut application)
}

fn init_logging() -> Result<(), anyhow::Error> {
    log4rs::init_file("./config/log.yaml", Deserializers::default())
}
