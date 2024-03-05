use std::{fs, io};
use std::path::PathBuf;
use glob::{glob};
use walkdir::{DirEntry, WalkDir};
use rand::Rng;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
const DIR: &str = "E:/testland/";

fn main() -> io::Result<()> {
    // rename all files
    rename_dirs(DIR).unwrap();
    rename_files(DIR).unwrap();
    Ok(())
}

fn rename_files(directory: &str) -> io::Result<()> {
    let results = glob(format!("{}**/*.*", directory).as_str()).unwrap();
    for entry in results
    {
        let path = entry.unwrap();
        println!("{}", path.to_str().unwrap());
        let new_path = get_new_path(path.clone());
        println!("{}\n", new_path);
        if fs::rename(path.clone(), new_path.clone()).is_err() && fs::rename(path.clone(), new_path.clone()).is_err() {
            println!("Error!\n\n")
        }
    }
    Ok(())
}

fn rename_dirs(start_directory: &str) -> io::Result<()> {
    for entry in WalkDir::new(start_directory)
        .contents_first(true).min_depth(1)
        .into_iter()
        .filter_entry(|e| !is_file(e)) {
        let path = entry.unwrap().into_path();
        println!("old: {}", path.to_str().unwrap());
        let new_path = get_new_path(path.clone());
        println!("new: {}\n", new_path);
        if fs::rename(path.clone(), new_path.clone()).is_err() && fs::rename(path.clone(), new_path.clone()).is_err() {
            println!("Error!\n\n")
        }
    }
    Ok(())
}

fn get_new_path(path: PathBuf) -> String {
    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen();
    format!("{}/{:?}", path.parent().unwrap().to_str().unwrap(), md5::compute(
        format!("{}{}", path.file_name().unwrap().to_str().unwrap(), random)))
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}