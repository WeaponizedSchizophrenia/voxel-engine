use log::LevelFilter;
use log4rs::{
    append::console::{ConsoleAppender, Target},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use super::Package;

/// Initializes logging.
#[derive(Default)]
pub struct LoggingInitPackage(Option<String>);

impl LoggingInitPackage {
    /// Initializes logging with a custom config file.
    ///
    /// # Arguments
    /// * `path` - The path to the config file.
    #[allow(unused)]
    pub fn with_custom_config(path: impl Into<String>) -> Self {
        Self(Some(path.into()))
    }
}

impl Package for LoggingInitPackage {
    fn initialize(&mut self, _app: &mut crate::application::Application) {
        match &self.0 {
            Some(path) => {
                if let Err(e) = log4rs::init_file(path, Default::default()) {
                    eprintln!("Failed to initialize logging: {e}");
                    println!("The application will continue without logging.");
                }
            }
            None => {
                let config = match Config::builder()
                    .appender(
                        Appender::builder().build(
                            "stdout",
                            Box::new(
                                ConsoleAppender::builder()
                                    .target(Target::Stdout)
                                    .encoder(Box::<PatternEncoder>::default())
                                    .build(),
                            ),
                        ),
                    )
                    .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
                {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        eprintln!("Failed to create logging config: {e}");
                        return;
                    }
                };
                if let Err(e) = log4rs::init_config(config) {
                    eprintln!("Failed to initialize logging: {e}");
                    println!("The application will continue without logging.");
                }
            }
        }
    }
}
