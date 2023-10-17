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
        let entry_path = entry.path();
        // Do not move folder if it's our own
        if entry_path != hide_folder { 
            let dest_path = hide_folder.join(entry_path.file_name().unwrap());
            fs::rename(entry_path, dest_path)?;
        }
    }

    Ok(())
}

pub fn show_files() -> std::io::Result<()> {
    let desktop_path = dirs::desktop_dir().expect("Failed to get desktop path");
    let documents_path = dirs::document_dir().expect("Failed to get documents path");
    let hide_folder = documents_path.join("HiddenDesktopFiles");

    for entry in fs::read_dir(hide_folder.clone())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let dest = desktop_path.join(path.file_name().unwrap());
            fs::rename(path, dest)?;
        }
    }

    // fs::remove_dir(hide_folder)?;

    Ok(())
}