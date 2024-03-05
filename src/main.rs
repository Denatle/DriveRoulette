use std::{fs, io};
use std::path::PathBuf;
use std::time::{Instant};
use walkdir::{WalkDir};
use clap::Parser;
use threadpool::ThreadPool;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

const DIR: &str = "E:/testland/";

fn main() -> io::Result<()> {
    let cpus = num_cpus::get();
    println!("{}", cpus);

    let now = Instant::now();
    // let args = Cli::parse();
    let path = PathBuf::from(DIR);

    let pool = ThreadPool::new(cpus);
    let paths = fs::read_dir(path.clone()).unwrap();
    for path in paths {
        let dir = path.unwrap();
        pool.execute(move || rename_tree(dir.path(), usize::MAX).unwrap())
    }
    pool.join();
    
    rename_tree(path, 2).unwrap();
    let now2 = now.elapsed();
    println!("{:?}", now2);
    println!("Took {} seconds", now2.as_secs());
    Ok(())
}

fn rename_tree(start_directory: PathBuf, max_depth: usize) -> io::Result<()> {
    for entry in WalkDir::new(start_directory)
        .contents_first(true).min_depth(1).max_depth(max_depth)
        .into_iter() {
        let path = entry.unwrap().into_path();
        // println!("{:?}", path);
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