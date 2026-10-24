use macroquad::prelude::*;

#[macroquad::main("dino game")]
async fn main() {
    let ground_y = 350.0;

    // dino properties
    let mut dino_y = ground_y - 80.0;
    let mut vel_y = 0.0;

    let dino_x = 100.0;
    let dino_width = 40.0;
    let dino_height = 80.0;

    let gravity = 1800.0;
    let jump_strength = -650.0;

    // cactus properties
    let cactus_width = 25.0;
    let cactus_height = 50.0;
    let mut cactus_x = screen_width() * 0.8;
    let cactus_speed = 350.0;

    // game state
    let mut game_over = false;

    loop {
        let delta = get_frame_time();

        if !game_over {
            // jump only when dino is on the ground
            if (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up)) && dino_y >= ground_y - dino_height {
                vel_y = jump_strength;
            }

            // apply gravity and update dino position
            vel_y += gravity * delta;
            dino_y += vel_y * delta;

            // stop dino on landing
            if dino_y > ground_y - dino_height {
                dino_y = ground_y - dino_height;
                vel_y = 0.0;
            }

            // move cactus to the left
            cactus_x -= cactus_speed * delta;

            // respawn cactus after it leaves screen
            if cactus_x + cactus_width < 0.0 {
                cactus_x = screen_width() + macroquad::rand::gen_range(150.0, 400.0);
            }

            // rectangle used for collision detection
            let dino_rect = Rect::new(
                dino_x, dino_y,
                dino_width, dino_height,
            );

            let cactus_rect = Rect::new(
                cactus_x, ground_y - cactus_height,
                cactus_width, cactus_height,
            );

            // detect collision
            if dino_rect.overlaps(&cactus_rect) {
                game_over = true;
            }
        }
        else {
            // restart the game
            if is_key_pressed(KeyCode::R) {
                dino_y = ground_y - dino_height;
                vel_y = 0.0;
                cactus_x = screen_width() * 0.8;
                game_over = false;
            }
        }

        // clear the screen with a light background
        clear_background(WHITE);

        // draw the ground
        draw_line(
            0.0, ground_y,
            screen_width(), ground_y,
            2.0, BLACK,
        );

        // draw dino body
        draw_rectangle(
            dino_x, dino_y,
            dino_width, dino_height,
            BLACK,
        );

        // draw dino head
        draw_rectangle(
            dino_x + 30.0, dino_y - 25.0,
            30.0, 30.0,
            BLACK,
        );

        // draw dino eye
        draw_circle(
            dino_x + 50.0, dino_y - 15.0,
            3.0, WHITE,
        );

        // draw cactus
        draw_rectangle(
            cactus_x, ground_y - cactus_height,
            cactus_width, cactus_height,
            BLACK
        );

        // draw cactus arms
        draw_rectangle(
            cactus_x - 8.0, ground_y - 35.0,
            8.0, 6.0,
            BLACK,
        );

        draw_rectangle(
            cactus_x - 8.0, ground_y - 45.0,
            6.0, 16.0,
            BLACK,
        );

        // title
        if game_over {
            draw_text(
                "GAME OVER",
                screen_width() / 2.0 - 100.0,
                120.0, 40.0,
                RED,
            );

            draw_text(
                "press R to restart",
                screen_width() / 2.0 - 100.0,
                160.0, 25.0,
                BLACK,
            );
        }
        else {
            draw_text(
                "press SPACE/UP to jump",
                20.0, 35.0, 25.0,
                BLACK,
            );
        }
        
        // wait and render the next frame
        next_frame().await;
    }
}