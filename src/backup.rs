use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use std::{thread, time::Duration};
use sysinfo::{System, Disks};
use chrono::Local;
use crate::gui::AppConfig;
use crate::event_manager::play_sound;

pub fn perform_backup() -> io::Result<()> {
    // Load the configuration settings from the default config
    let config = AppConfig::load();

    // Define the source path and the file extension for filtering files
    let src: &Path = Path::new(config.source_dir.as_str());
    let mut target_ext = config.file_extension.as_str();
    if target_ext.is_empty() {
       target_ext = "all";
    }

    // Calculate the total space required to copy the files
    let required_space = get_dir_size(src, target_ext)?;

    // Find a USB drive with enough available space (retrying if necessary)
    let dst = find_usb(required_space)?;

    let time = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let new_name = format!("{}_{}", src.file_name().unwrap().to_str().unwrap(), time);
    let dest_new_path = dst.join(new_name);

    // Check if both source and destination paths exist 
    if src.exists() && dst.exists() && !config.source_dir.is_empty() {
        fs::create_dir(dest_new_path.clone())?;
        copy_dir_recursive(src, dest_new_path.as_path(), target_ext)?;
    } else {
        // If the source path is invalid, play a sound and return an error
        println!("Please, specify a valid folder to backup.");
        play_sound("valid_folder.mp3");
        return Err(io::Error::new(ErrorKind::NotFound, "Source path not found"));
    }

    Ok(())
}

fn find_usb(min_space: u64) -> io::Result<std::path::PathBuf> {
    loop {
        let mut system = System::new_all();
        system.refresh_all();
        
        let disks = Disks::new_with_refreshed_list();

        // Iterate over all disks and check if there is a removable disk with enough available space
        for disk in &disks {
            if disk.is_removable() && disk.available_space() > min_space && !disk.is_read_only() {
                return Ok(disk.mount_point().to_path_buf());
            }
        }

        // If no suitable USB drive is found
        println!("External disk not found. Please, connect ones to start backup.");
        play_sound("disk_not_found.mp3");
        thread::sleep(Duration::from_secs(10)); // Wait for 10 seconds before retrying
    }
}

/// Calculates the total size of files to be copied in a directory (recursively)
fn get_dir_size(path: &Path, target_ext: &str) -> io::Result<u64> {
    let mut total_size = 0;

    // Iterate over all entries in the directory
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            total_size += get_dir_size(&path, target_ext)?;
        } else {
            if target_ext == "all" {
                total_size += path.metadata()?.len();
            } else if let Some(ext) = path.extension() {
                if ext.to_str().unwrap() == target_ext {
                    total_size += path.metadata()?.len();
                }
            }
        }
    }

    Ok(total_size)
}

fn copy_dir_recursive(src: &Path, dst: &Path, target_ext: &str) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir(dst)?;
    }

    // Iterate over all entries in the source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path, target_ext)?;
        } else {
            if target_ext == "all" {
                fs::copy(&path, &dest_path)?;
            } else if let Some(ext) = path.extension() {
                if ext.to_str().unwrap() == target_ext {
                    fs::copy(&path, &dest_path)?;
                }
            }
        }
    }

    Ok(())
}
