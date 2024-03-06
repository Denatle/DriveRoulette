use macroquad::prelude::*;

pub(crate) async fn ui() {
    loop {
        clear_background(BLACK);


        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let radius = (screen_width() / 3.0) - screen_height() / 1.5;

        draw_circle(center_x, center_y, radius, LIGHTGRAY);

        draw_text("DRIVE ROULETTE", 20.0, 20.0, 30.0, DARKGRAY);

        let chamber_offset: f32 = radius / 1.8;
        let initial_x: f32 = center_x;
        let initial_y: f32 = center_y + chamber_offset;
        let text_offset: f32 = -10.0;

        let speed = 2.0;

        for i in 0..5 + 1 {
            let c: f32 = std::f32::consts::FRAC_PI_3 * i as f32 + (get_time() as f32) * speed;

            let rotation_x = c.cos() * (initial_x - center_x) - c.sin() * (initial_y - center_y) + center_x;
            let rotation_y = c.sin() * (initial_x - center_x) - c.cos() * (initial_y - center_y) + center_y;
            draw_circle(rotation_x, rotation_y, radius / 4.0, GRAY);
            draw_text(&i.to_string(), rotation_x+text_offset, rotation_y-text_offset, 50.0, DARKGRAY);
        }

        next_frame().await
    }
}