use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;
use std::path::{Path, PathBuf};
use std::io::{self};
use std::fs::File;
use std::fs;
use colored::*;

#[allow(dead_code)]
fn optional_zipper(){
    // let mut zip_input: String = String::new();
    // loop
    // {   
    //     println!("\ndo you want to zip the archived folder? y/n");
    //     io::stdout().flush()?;
    //     match io::stdin().read_line(&mut zip_input)
    //     {
    //         Ok(_) => 
    //         {
    //             match zip_input.trim().to_lowercase().as_str() 
    //             {
    //                 "yes"|"y" => 
    //                 {
    //                     println!("\nZipping folder {}, please wait", backup_dir.file_name().expect("Cannot extract directory name").to_string_lossy().blue());
    //                     let _ = zip_archived_folder(&backup_dir);
    //                     break;
    //                 },
    //                 "no"|"n" =>
    //                 {
    //                     break;
    //                 }
    //                 _ => 
    //                 {
    //                     println!("\n{}","Enter a Valid input! y/n".red());
    //                     continue;
    //                 }
    //             }
    //         }
    //         Err(_error) => 
    //         {
    //             println!("Couldnt Read input!");
    //             continue;
    //         }
    //     }
    // }
}

pub fn start_zipping(backup_dir: &PathBuf) -> io::Result<()>
{
    println!("\nZipping folder {}, please wait..", backup_dir.file_name().expect("Cannot extract directory name").to_string_lossy().blue());
    let _ = zip_archived_folder(&backup_dir);
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