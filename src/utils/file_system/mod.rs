pub mod read;

use std::{io, path::PathBuf};

/// The relative path to the assets directory.
const ASSETS_DIR: &str = "./assets";
/// Returns a `PathBuf` to the assets directory.
pub fn get_asset_dir() -> PathBuf {
    PathBuf::from(ASSETS_DIR)
}

/// Reads the shader with the given name.
pub fn read_wgsl_shader(name: &str) -> io::Result<String> {
    let mut path = get_asset_dir();
    path.push("shaders");
    path.push(name);
    path.set_extension("wgsl");
    read::read_text(path)
}
