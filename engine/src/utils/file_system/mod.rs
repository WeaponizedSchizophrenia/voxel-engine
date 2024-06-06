pub mod read;
pub mod write;

use std::{fs, io, path::PathBuf};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

/// Reads the asset config and returns the result.
pub fn read_asset_config(config_type: &str, name: &str) -> io::Result<String> {
    let mut path = get_asset_dir();
    path.push("configs");
    path.push(config_type);
    path.push(name);
    path.set_extension("ron");
    read::read_text(path)
}

/// Returns a parallel iterator over all asset config files for the provided config type.
pub fn iter_all_asset_configs(
    config_type: &str,
) -> io::Result<impl ParallelIterator<Item = String> + '_> {
    let mut path = get_asset_dir();
    path.push("configs");
    path.push(config_type);

    let directory = fs::read_dir(path)?;

    Ok(directory
        .into_iter()
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                let name = entry.file_name();
                let name = match name.to_str() {
                    Some(name) => name,
                    None => {
                        log::error!("Failed to read directory entry");
                        return None;
                    }
                };
                match read_asset_config(config_type, name) {
                    Ok(cfg) => Some(cfg),
                    Err(e) => {
                        log::error!("Failed to read config: {e}");
                        None
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read directory entry: {e}");
                None
            }
        }))
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
