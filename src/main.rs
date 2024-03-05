use std::{fs, io};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use rand::Rng;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

// const DIR: &str = "E:/testland/";

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let path = args.path;
    // rename_dirs(path.clone()).unwrap();
    // rename_files(path).unwrap();
    Ok(())
}

fn rename_files(start_directory: PathBuf) -> io::Result<()> {
   for entry in WalkDir::new(start_directory)
        .contents_first(true).min_depth(1)
        .into_iter()
        .filter_entry(is_file) {
        let path = entry.unwrap().into_path();
        println!("old: {}", path.to_str().unwrap());
        let new_path = get_new_path(path.clone());
        println!("new: {}\n", new_path);
        rename(path, PathBuf::from(new_path));
    }
    Ok(())
}

fn rename_dirs(start_directory: PathBuf) -> io::Result<()> {
    for entry in WalkDir::new(start_directory)
        .contents_first(true).min_depth(1)
        .into_iter()
        .filter_entry(|e| !is_file(e)) {
        let path = entry.unwrap().into_path();
        println!("old: {}", path.to_str().unwrap());
        let new_path = get_new_path(path.clone());
        println!("new: {}\n", new_path);
        rename(path, PathBuf::from(new_path));
    }
    Ok(())
}

fn get_new_path(path: PathBuf) -> String {
    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen();
    format!("{}/{:?}", path.parent().unwrap().to_str().unwrap(), md5::compute(
        format!("{}{}", path.file_name().unwrap().to_str().unwrap(), random)))
}

fn rename(path: PathBuf, new_path: PathBuf) {
    if fs::rename(path.clone(), new_path.clone()).is_err() && fs::rename(path.clone(), new_path.clone()).is_err() {
        println!("Error!\n\n")
    }
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}