use std::path::{Path, PathBuf};
use std::io::{self,Write};
use std::fs;
use colored::*;
use crate::constants::ZIP_EXTENSION;

pub fn clean_project(project_dir:&Path, backup_dir: &PathBuf)
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