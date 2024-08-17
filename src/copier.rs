use std::path::Path;
use std::io::{self};
use std::fs;
use colored::*;

use crate::git_handler;
use crate::constants::REQUIRED_FOLDERS;


pub fn copy_project_files(source: &Path, destination: &Path) -> io::Result<()>
{
    fs::create_dir_all(destination).expect("Couldnt complete directory creation!");

    git_handler::move_git(&source, &destination);

    for &dir in &REQUIRED_FOLDERS
    {
        let source_path = source.join(dir);
        let destination_path = destination.join(dir);
        println!
        (
            "Moving {} to {}",
            source_path.to_string_lossy().blue(),
            destination_path.to_string_lossy().green()
        );

        if source_path.is_dir()        
        {
            replicate_folder_recursively(&source_path, &destination_path)?;
        }
    }
    return Ok(());
}

pub fn replicate_folder_recursively(source: &Path, destination: &Path) -> io::Result<()>
{
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? 
    {
        let entry = entry?;
        let entry_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if entry_path.is_dir()
        {
            let _ = replicate_folder_recursively(&entry_path, &destination_path);
        }
        else
        {
            println!
            (
                "Moving {} to {}",
                entry_path.to_string_lossy().blue(),
                destination_path.to_string_lossy().green()
            );
            let _ = fs::copy(&entry_path, &destination_path);
        }
    }
    return Ok(());
}