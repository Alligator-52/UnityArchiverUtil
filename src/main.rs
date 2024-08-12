use std::env;
use std::fs;
use std::io::{self,Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;
use zip::write::FileOptions;

const REQUIRED_FOLDERS: [&str; 4] = ["Assets","Packages", "ProjectSettings", "UserSettings"];
fn main() {
    if(is_unity_dir(Path::new(r"D:\Dx\Project\HoloLens Runtime - Testing")))
    {
        println!("It is working");
    }
    else{
        println!("Not working!");
    }
}

fn is_unity_dir(dir: &Path) -> bool
{
    return REQUIRED_FOLDERS.iter().all(|&d| dir.join(d).is_dir());
}
