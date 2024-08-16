use std::env;
use std::fs;
use std::fs::File;
use std::io::{self,Write};
use std::path::{Path, PathBuf};
use colored::*;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

//to do, add option to delete og files
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
        println!("{}", entry.path().to_string_lossy().yellow());
    }
    if !is_unity_project(path)
    {
        println!("\nThe provided directory is not a unity project!");
        return  Ok(());
    }

    let backup_dir = path.join(name + "_Archive");
    //println!("\narchive directory name: {:?}", &backup_dir);
    let _ = copy_project_files(path, &backup_dir);

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
                        println!("\nZipping folder {}", backup_dir.file_name().expect("Cannot extract directory name").to_string_lossy().blue());
                        let _ = zip_archived_folder(&backup_dir);
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
            println!("Missing folder: {}", folder.red());
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
            let _ = replicate_folder_recursively(&entry_path, &destination_path);
        }
        else
        {
            let _ = fs::copy(&entry_path, &destination_path);
        }
    }
    return Ok(());
}

fn zip_archived_folder(folder_to_zip :&PathBuf) -> io::Result<()>
{
    let zip_file_path = folder_to_zip.with_extension("zip");
    let zip_file = File::create(&zip_file_path)?;
    let mut zip = ZipWriter::new(zip_file);
    let zip_option = FileOptions::default().compression_method(CompressionMethod::ZSTD);

    for item in fs::read_dir(folder_to_zip)? 
    {
        let item = item?;
        let path = item.path();

        if path.is_file()
        {
            let file_name = match path.strip_prefix(folder_to_zip) 
            {
                Ok(p) => p.to_str().unwrap(),
                Err(e) => {
                    eprintln!("Failed to strip prefix: {}", e);
                    return Err(io::Error::new(io::ErrorKind::Other, "Prefix stripping failed"));
                }
            };
            let mut file = File::open(&path)?;

            zip.start_file(file_name, zip_option)?;

            io::copy(&mut file, &mut zip)?;
        }
        else if path.is_dir()
        {
            zip_directory(&path, &mut zip, &zip_option, folder_to_zip)?;
        }
    }
    zip.finish()?;
    println!
    (
        "{} success ziped to {}", 
        folder_to_zip.to_string_lossy().yellow(), 
        zip_file_path.to_string_lossy().green()
    );

    return Ok(());
}

fn zip_directory(directory: &Path, zip: &mut ZipWriter<File>, zip_option : &FileOptions, base_directory: &Path) -> io::Result<()>
{
    for item in fs::read_dir(directory)?
    {
        let item = item?;
        let path = item.path();

        if path.is_file()
        {
            let file_name = match path.strip_prefix(base_directory) 
            {
                Ok(p) => p.to_str().unwrap(),
                Err(e) => {
                    eprintln!("Failed to strip prefix: {}", e);
                    return Err(io::Error::new(io::ErrorKind::Other, "Prefix stripping failed"));
                }
            };
            let mut file = File::open(&path)?;
            let _ = zip.start_file(file_name, *zip_option)?;
            let _ = io::copy(&mut file, zip);
        }
        else if path.is_dir()
        {
            zip_directory(&path, zip, zip_option, base_directory)?;    
        }
    }
    return Ok(());
}