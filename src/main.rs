use std::env;
use std::fs;
use std::io::{self,Write};
use std::path::{Path};
// use std::process::Command;
// use flate2::write::GzEncoder;
// use flate2::Compression;
// use std::fs::File;
// use std::io::prelude::*;
// use zip::write::FileOptions;

const REQUIRED_FOLDERS: [&str; 4] = ["Assets","Packages", "ProjectSettings", "UserSettings"];
fn main() -> io::Result<()> 
{
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 
    {
        println!("Empty!");
        return Ok(());
    }  

    let path = Path::new(&args[1]);

    if !path.is_dir()
    {
        println!("The provided path is not a direcrtory");
        return Ok(());
    } 

    if !is_unity_project(path)
    {
        println!("The provided directory is not a unity project!");
        return  Ok(());
    }

    let backup_dir = path.join("Archive");
    copy_project_files(path, &backup_dir);
    return Ok(());
}

fn is_unity_project(dir: &Path) -> bool
{
    return REQUIRED_FOLDERS.iter().all(|&d| dir.join(d).is_dir());
}

fn copy_project_files(source: &Path, destination: &Path)
{
    println!("This method is working");
}
