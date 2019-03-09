use std::path::Path;
use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn create_file_with_content(path: &std::path::PathBuf, content:&str) -> std::io::Result<()> {
    if Path::new(path).exists() {return Ok(())}
    let file = OpenOptions::new().append(false).create(true).write(true).truncate(false).open(path);
    match file {
        Ok(mut f) => f.write_all(content.as_bytes()),
        Err(e) => Err(e)
    }
}
