use std::path::Path;
use std::io::{self};
use std::fs;
use colored::*;

use crate::copier;

pub fn move_git(source: &Path, destination: &Path)
{
    if !is_git(source)
    {
        return;
    }
    println!("{}","Git repository found in the project".yellow());
    let _= copy_git(source, destination);
}

fn copy_git(source: &Path, destination: &Path) -> io::Result<()>
{
    println!("{}\n","Copying Git repository".yellow());

    let git_folder = source.join(".git");
    let git_dest = destination.join(".git");

    if git_folder.is_dir()
    {
        copier::replicate_folder_recursively(&git_folder, &git_dest)?;
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

fn is_git(directory: &Path) -> bool 
{
    let git_path = directory.join(".git");

    return git_path.is_dir();
}