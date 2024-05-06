use log4rs::config::Deserializers;

fn main() {
    match init_logging() {
        Ok(_) => log::info!("Logging intialized"),
        Err(e) => {
            eprintln!("Failed to initialize logging: {}", e);
            println!("The application will continue without logging.");
        }
    }
}

fn init_logging() -> Result<(), anyhow::Error> {
    log4rs::init_file("./config/log.yaml", Deserializers::default())
}
