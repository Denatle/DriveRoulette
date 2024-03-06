use std::f32::consts::{FRAC_PI_3, PI};
use std::path::PathBuf;
use std::thread;

use macroquad::color::*;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

use crate::disk;

const ANIM_TIME: f32 = 12.0;
const DEFAULT_SPEED: f32 = 7.0;

pub(crate) async fn ui() {
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
            let last_drive_copy = last_drive.clone();
            thread::spawn(move || { pick(last_drive_copy) });
        }
        speed = DEFAULT_SPEED + (ANIM_TIME - time as f32);

        clear_background(BLACK);
        if locked {
            draw_text(format!("Formatting drive {:?}...", last_drive.clone()).as_str(), 20.0, screen_height() - 20.0, 30.0, RED)
        }

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let radius = (screen_width() / 3.0) - screen_height() / 1.5;

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
            if ((c % (PI * 2.0)) > 2.3) && ((c % (PI * 2.0)) < 3.3) {
                draw_circle(rotation_x, rotation_y, radius / 4.0, YELLOW);
                last_drive = disks.get(i).unwrap().to_path_buf();
                // draw_text((c % (PI * 2.0)).to_string().as_str(), rotation_x - 30.0, rotation_y + 15.0, 50.0, DARKGRAY);
                // draw_text(speed.to_string().as_str(), rotation_x - 30.0, rotation_y + 50.0, 50.0, DARKGRAY);
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