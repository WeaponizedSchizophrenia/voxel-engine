use std::{io, path::Path};

/// Writes the given text to the given path.
///
/// Note: If the file already exists, it will be overwritten.
/// Or if it does not exist one will be created.
pub fn write_text<P: AsRef<Path>>(path: P, text: &str) -> io::Result<()> {
    std::fs::write(path, text)?;

    Ok(())
}
