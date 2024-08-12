use std::env;
use std::fs;
use std::io::{self,Write};
use std::path::{Path, PathBuf};

const REQUIRED_FOLDERS: [&str; 4] = ["Assets","Packages", "ProjectSettings", "UserSettings"];
fn main() -> io::Result<()> 
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Empty!");
        return Ok(());
    }  

    let path = Path::new(&args[1]);

    if !path.is_dir()
    {
        println!("Invalid Directory Path!");
        return Ok(());
    } 
    println!("Contents of the provided directory:");
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        println!("{}", entry.path().display());
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

// fn is_unity_project(dir: &Path) -> bool
// {
//     return REQUIRED_FOLDERS.iter().all(|&d| dir.join(d).is_dir());
// }
fn is_unity_project(dir: &Path) -> bool {
    for &folder in &REQUIRED_FOLDERS {
        let folder_path = dir.join(folder);
        println!("Checking for folder: {}", folder_path.display());
        if !folder_path.is_dir() {
            println!("Missing folder: {}", folder);
            return false;
        }
    }
    true
}

fn copy_project_files(source: &Path, destination: &Path) -> io::Result<()>
{
    fs::create_dir_all(destination).expect("Couldnt complete directory creation!");
    for &dir in &REQUIRED_FOLDERS
    {
        let source_path = source.join(dir);
        let destination_path = destination.join(dir);
        if source_path.is_dir()        
        {
            fs::create_dir_all(&destination_path)?;
            fs::copy(source_path,destination_path)?;
        }
    }
    return Ok(());
}
