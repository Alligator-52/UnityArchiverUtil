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
    let mut name:String = String::new();
    if let Some(folder_name) = path.file_name()
    {
        name = folder_name.to_string_lossy().to_string();
    }

    println!("\nFolder name: {name}\n");
    // println!("Path i am getting : {:?}",path.display());

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
        println!("\nThe provided directory is not a unity project!");
        return  Ok(());
    }

    let backup_dir = path.join(name + "_Archive");
    println!("\narchive directory name: {:?}", &backup_dir);
    copy_project_files(path, &backup_dir);

    let mut zip_input: String = String::new();

    loop
    {
        println!("do you want to zip the archived folder? y/n \n");
        match io::stdin().read_line(&mut zip_input)
        {
            Ok(_) => 
            {
                match zip_input.trim().to_lowercase().as_str() 
                {
                    "yes"|"y" => 
                    {
                        println!("\nThe folder will be zipped");
                        break;
                    },
                    "no"|"n" =>
                    {
                        println!("\nWill not be zipped");
                        break;
                    }
                    _ => 
                    {
                        println!("\nenter a valid input");
                        continue;
                    }
                }
            }
            Err(_error) => 
            {
                println!("Couldnt Read input!");
                continue;
            }
        }
    }
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
            replicate_folder_recursively(&source_path, &destination_path)?;
        }
    }
    return Ok(());
}

fn replicate_folder_recursively(source: &Path, destination: &Path) -> io::Result<()>
{
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? 
    {
        let entry = entry?;
        let entry_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if entry_path.is_dir()
        {
            replicate_folder_recursively(&entry_path, &destination_path);
        }
        else
        {
            fs::copy(&entry_path, &destination_path);
        }
    }
    return Ok(());
}