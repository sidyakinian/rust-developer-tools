extern crate dirs;
extern crate structopt;

use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

pub fn hide_files() -> std::io::Result<()> {
    let desktop_path = dirs::desktop_dir().expect("Failed to get desktop path");
    let documents_path = dirs::document_dir().expect("Failed to get documents path");
    let hide_folder = documents_path.join("HiddenDesktopFiles");

    if !hide_folder.exists() {
        fs::create_dir(&hide_folder)?;
    }

    for entry in fs::read_dir(desktop_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let dest = hide_folder.join(path.file_name().unwrap());
            fs::rename(path, dest)?;
        }
    }

    Ok(())
}