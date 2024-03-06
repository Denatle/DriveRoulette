use std::{fs, io};
use std::path::PathBuf;
use std::time::Instant;
use threadpool::ThreadPool;
use walkdir::WalkDir;

pub(crate) fn start_rename(path: PathBuf) {
    let now = Instant::now();
    let cpus = num_cpus::get();
    println!("{}", cpus);
    rename_tree(path.clone(), 2).unwrap();

    let pool = ThreadPool::new(cpus);
    let paths = fs::read_dir(path.clone()).unwrap();
    for path in paths {
        let dir = path.unwrap();
        pool.execute(move || rename_tree(dir.path(), usize::MAX).unwrap())
    }
    pool.join();

    let now2 = now.elapsed();
    println!("{:?}", now2);
    println!("Took {} seconds", now2.as_secs());
}

fn rename_tree(start_directory: PathBuf, max_depth: usize) -> io::Result<()> {
    for entry in WalkDir::new(start_directory)
        .contents_first(true).min_depth(1).max_depth(max_depth)
        .into_iter() {
        let err = entry.is_err();
        if err { continue; }
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
