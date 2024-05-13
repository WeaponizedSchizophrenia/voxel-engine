pub mod read;
pub mod write;

use std::{io, path::PathBuf};

/// The relative path to the assets directory.
pub const ASSETS_DIR: &str = "./assets";
/// Returns a `PathBuf` to the assets directory.
pub fn get_asset_dir() -> PathBuf {
    PathBuf::from(ASSETS_DIR)
}
pub const CONFIG_PATH: &str = "./config/config.yml";

/// Reads the config and returns the result.
pub fn read_config() -> io::Result<String> {
    read::read_text(CONFIG_PATH)
}

pub fn read_asset_config(name: &str) -> io::Result<String> {
    let mut path = get_asset_dir();
    path.push(name);
    path.set_extension("ron");
    read::read_text(path)
}

/// Writes the serialized config and returns the result.
pub fn write_config(config: &str) -> io::Result<()> {
    write::write_text(CONFIG_PATH, config)
}

/// Reads the shader with the given name.
pub fn read_wgsl_shader(name: &str) -> io::Result<String> {
    let mut path = get_asset_dir();
    path.push("shaders");
    path.push(name);
    path.set_extension("wgsl");
    read::read_text(path)
}
