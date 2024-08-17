use std::env;
use std::fs;
use std::fs::File;
use std::io::{self,Write};
use std::path::{Path, PathBuf};
use colored::*;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

//mod ansi_support;
//to do, add option to delete og files
const REQUIRED_FOLDERS: [&str; 4] = ["Assets","Packages", "ProjectSettings", "UserSettings"];
const ZIP_EXTENSION: &str = "zip";
const ARCHIVE:&str = "_Archive";

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

    //println!("\nFolder name: {name}\n");
    
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

    let _ = copy_project_files(path, &backup_dir);

    let _ = start_zipping(&backup_dir);

    clean_project(&path, &backup_dir);
    
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

fn copy_project_files(source: &Path, destination: &Path) -> io::Result<()>
{
    fs::create_dir_all(destination).expect("Couldnt complete directory creation!");

    let _ = move_git(&source, &destination);

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

fn move_git(source: &Path, destination: &Path) -> io::Result<()>
{
    if !is_git(source)
    {
        return Ok(());
    }
    
    println!("{}\n","Copying Git repository".yellow());

    let git_folder = source.join(".git");
    let git_dest = destination.join(".git");

    if git_folder.is_dir()
    {
        replicate_folder_recursively(&git_folder, &git_dest)?;
    }

    let git_ignore = source.join(".gitignore");
    if git_ignore.is_file()
    {
        let git_ignore_dest = destination.join(".gitignore");
        let _ = fs::copy(git_ignore, git_ignore_dest)?;
    }

    let git_att = source.join(".gitattributes");
    if git_att.is_file()
    {
        let git_att_dest = destination.join(".gitattributes");
        let _ = fs::copy(git_att, git_att_dest)?;
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

fn start_zipping(backup_dir: &PathBuf) -> io::Result<()>
{
    let mut zip_input: String = String::new();
    loop
    {   
        println!("\ndo you want to zip the archived folder? y/n");
        io::stdout().flush()?;
        match io::stdin().read_line(&mut zip_input)
        {
            Ok(_) => 
            {
                match zip_input.trim().to_lowercase().as_str() 
                {
                    "yes"|"y" => 
                    {
                        println!("\nZipping folder {}, please wait", backup_dir.file_name().expect("Cannot extract directory name").to_string_lossy().blue());
                        let _ = zip_archived_folder(&backup_dir);
                        break;
                    },
                    "no"|"n" =>
                    {
                        break;
                    }
                    _ => 
                    {
                        println!("\n{}","Enter a Valid input! y/n".red());
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
    println!("{}","Successfully Zipped Archive Folder".green());
    return Ok(());
}

fn zip_archived_folder(folder_to_zip :&PathBuf) -> io::Result<()>
{
    println!("\n{}","Running Archive Zip..".yellow());
    let zip_file_path = folder_to_zip.with_extension("zip");
    let zip_file = File::create(&zip_file_path)?;
    let mut zip = ZipWriter::new(zip_file);
    let zip_option = FileOptions::default().compression_method(CompressionMethod::ZSTD); //DEFLATE - best compression, BZIP2 - lesser compression, faster, ZSTD - fastest compression

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

fn start_deleting(project_dir:&Path, backup_dir: &PathBuf) -> io::Result<()>
{
    println!("\n{}","Starting Deletion Process, please wait..".yellow());
    for item in fs::read_dir(project_dir)?
    {
        let item = item?;
        let path = item.path();
        let backup_dir = backup_dir.as_path();
        if (path == backup_dir) ||
         (path.extension().map_or(false, |ext| ext == ZIP_EXTENSION))
        {
            continue;
        }
        println!("Deleting {}", path.to_string_lossy().red());
        if path.is_dir()
        {
            fs::remove_dir_all(&path)?;
        }
        else
        {
            fs::remove_file(&path)?;
        }
    }
    println!("\n{}","Project Cleanup Finshed!".green());
    return Ok(());
}

fn clean_project(project_dir:&Path, backup_dir: &PathBuf)
{
    let mut delete_input: String = String::new();
    loop
    {
        println!("\nDo you want to keep the original files: y/n");
        io::stdout().flush().expect("Couldnt flush stdout");
        match io::stdin().read_line(&mut delete_input) 
        {
            Ok(_) => 
            {
                match delete_input.trim().to_lowercase().as_str() 
                {
                    "no"|"n" => 
                    {
                        let _ = start_deleting(project_dir, backup_dir);
                        break;
                    }
                    "yes"|"y" => 
                    {
                        println!("{}","Keeping Original Files".green());
                        break;
                    }
                    _ => 
                    {
                        println!("\n{}","Enter a Valid input! y/n".red());
                        continue;
                    }
                }
            }
            Err(_er) =>
            {
                println!("Couldnt Read Input");
                continue;
            }
        }
    }
}

fn is_git(directory: &Path) -> bool 
{
    let git_path = directory.join(".git");

    return git_path.is_dir();
}