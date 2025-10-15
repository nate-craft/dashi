use std::{fs, io, path::PathBuf};

pub fn data_dir_file(added: &str) -> Result<PathBuf, io::Error> {
    data_dir().map(|dir| dir.join(added))
}

pub fn data_dir() -> Result<PathBuf, io::Error> {
    let dir = dirs::data_local_dir()
        .map(|dir| dir.join("dashi/"))
        .ok_or(io::Error::from(io::ErrorKind::NotFound));

    if let Ok(dir) = &dir {
        fs::create_dir_all(dir)?;
    }

    dir
}
