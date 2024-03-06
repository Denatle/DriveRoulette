#![windows_subsystem = "windows"]
mod disk;
mod interface;
mod windows;

use std::path::PathBuf;
use clap::Parser;
use macroquad::prelude::Conf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

// const DIR: &str = "E:/testland/";

fn window_conf() -> Conf {
    Conf {
        window_title: "DRIVE_ROULETTE".to_owned(),
        window_width: 700,
        window_height: 700,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), ()> {
    // let args = Cli::parse();
    // let path = args.path;

    windows::block_input();
    
    interface::ui().await;
    
    Ok(())
}
