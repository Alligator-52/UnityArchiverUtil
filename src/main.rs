use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;
use colored::*;


mod copier;
mod zipper;
mod git_handler;
mod cleaner;
mod constants;

use crate::constants::{REQUIRED_FOLDERS,ARCHIVE};
//mod ansi_support;

fn main() -> io::Result<()> 
{
    //ansi_support::enable_ansi_support().expect("Failed to convert colored codes");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Empty!");
        return Ok(());
    }  

    let path = Path::new(&args[1]);
    let mut name:String = String::new();
    if let Some(folder_name) = path.file_name()
    {
        name = folder_name.to_string_lossy().to_string();
    }
    
    if !path.is_dir()
    {
        println!("Invalid Directory Path!");
        return Ok(());
    } 
    println!("Contents of the provided directory:");
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        println!("{}", entry.path().to_string_lossy().yellow());
    }
    if !is_unity_project(path)
    {
        println!("\nThe provided directory is not a unity project!");
        return  Ok(());
    }

    let backup_dir = path.join(name.clone() + ARCHIVE);

    let _ = copier::copy_project_files(path, &backup_dir);

    let _ = zipper::start_zipping(&backup_dir);

    cleaner::clean_project(&path, &backup_dir);
    
    return Ok(());
}


fn is_unity_project(dir: &Path) -> bool 
{
    for &folder in &REQUIRED_FOLDERS {
        let folder_path = dir.join(folder);
        println!("Checking for folder: {}", folder_path.display());
        if !folder_path.is_dir() {
            println!("Missing folder: {}", folder.red());
            return false;
        }
    }
    true
}

