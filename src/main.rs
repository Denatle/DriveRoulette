use std::{fs, io};
use std::path::PathBuf;
use std::time::Instant;
use walkdir::{WalkDir};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

// const DIR: &str = "E:/testland/";

fn main() -> io::Result<()> {
    let now = Instant::now();
    let args = Cli::parse();
    let path = args.path;
    rename_tree(path.clone()).unwrap();
    let now2 = now.elapsed();
    println!("{:?}", now2);
    println!("Took {} seconds", now2.as_secs());
    Ok(())
}

fn rename_tree(start_directory: PathBuf) -> io::Result<()> {
    for entry in WalkDir::new(start_directory)
        .contents_first(true).min_depth(1)
        .into_iter() {
        let path = entry.unwrap().into_path();
        let new_path = get_new_path(path.clone());
        rename(path, PathBuf::from(new_path));
    }
    Ok(())
}

fn get_new_path(path: PathBuf) -> String {
    format!("{}/{:?}", path.parent().unwrap().to_str().unwrap(), md5::compute(
        path.file_name().unwrap().to_str().unwrap()))
}

fn rename(path: PathBuf, new_path: PathBuf) {
    let _ = fs::rename(path.clone(), new_path.clone()).is_err();
}