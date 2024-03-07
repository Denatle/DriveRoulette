#![windows_subsystem = "windows"]

mod disk;
mod interface;
mod windows;

use macroquad::miniquad::conf::Icon;
use macroquad::prelude::{Conf, load_image};


fn window_conf() -> Conf {
    Conf {
        window_title: "DRIVE_ROULETTE".to_owned(),
        window_width: 1920,
        window_height: 1080,
        high_dpi: false,
        fullscreen: true,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), ()> {
    interface::ui().await;
    
    #[cfg(not(debug_assertions))]
    windows::block_input();

    Ok(())
}
