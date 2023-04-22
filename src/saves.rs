// Just do what Factorio & Prison Architect do:
// https://savelocation.net/factorio
// https://savelocation.net/prison-architect

use std::{env, ffi::OsString, fs, io::Result, path::PathBuf};

#[cfg(target_os = "macos")]
fn get_save_dir() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap()).join("Library/Application Support/Rustic Yellow/saves")
}

#[cfg(target_os = "linux")]
fn get_save_dir() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap()).join(".Rustic Yellow/saves")
}

#[cfg(target_os = "windows")]
fn get_save_dir() -> PathBuf {
    PathBuf::from(env::var("appdata").unwrap()).join("Rustic Yellow\\saves")
}

pub struct SaveFile {
    pub path: PathBuf,
    pub name: String,
}

pub fn list_save_files() -> Result<Vec<SaveFile>> {
    let ext: OsString = OsString::from("sav");

    let dir = match fs::read_dir(get_save_dir()) {
        Ok(dir) => dir,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let mut files = Vec::new();

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.extension() == Some(&ext) && path.is_file() {
            if let Some(stem) = path.file_stem() {
                if let Some(name) = stem.to_str() {
                    let name = name.to_owned();
                    files.push(SaveFile { path, name });
                }
            }
        }
    }

    // Sort by last modified
    files.sort_by_cached_key(|save| {
        std::cmp::Reverse(
            save.path
                .metadata()
                .and_then(|meta| meta.modified())
                .ok()
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
        )
    });

    Ok(files)
}
