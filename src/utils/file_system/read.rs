use std::{fs::File, io, path::Path};

/// Reads the text content of the given file.
pub fn read_text<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let file = File::open(path)?;
    let content = io::read_to_string(file)?;
    Ok(content)
}
