use macroquad::prelude::*;

struct Cactus {
    x: f32,
    height: f32,
}

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
    let mut cacti = vec![
        Cactus {
            x: screen_width() * 0.8,
            height: 50.0,
        }
    ];
    let base_cactus_speed = 350.0;

    // score
    let mut score = 0.0;

    // game state
    let mut game_over = false;

    loop {
        let delta = get_frame_time();

        if !game_over {
            // increase score while playing
            score += delta * 10.0;

            // increase cactus speed as score rises
            let cactus_speed = (base_cactus_speed + score * 0.5).min(700.0);

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
            for cactus in cacti.iter_mut() {
                cactus.x -= cactus_speed * delta;
            }

            cacti.retain(|cactus| cactus.x + cactus_width >= 0.0);

            // spawn new cactus when rightmost cactus is far left enough
            let rightmost_cactus = cacti.iter().map(|cactus| cactus.x).reduce(f32::max);

            if let Some(rightmost_x) = rightmost_cactus {
                if rightmost_x < screen_width() - 100.0 {
                    let gap = macroquad::rand::gen_range(300.0, 500.0);
                    let start_x = screen_width() + gap;
                    
                    spawn_cactus_pattern(&mut cacti, start_x);
                }
            }
            else {
                // spawn a pattern if no cacti remain
                let start_x = screen_width() + macroquad::rand::gen_range(300.0, 500.0);
                spawn_cactus_pattern(&mut cacti, start_x);
            }

            // rectangle used for collision detection
            let dino_rect = Rect::new(
                dino_x, dino_y,
                dino_width, dino_height,
            );

            for cactus in cacti.iter() {
                let cactus_rect = Rect::new(
                    cactus.x, ground_y - cactus.height,
                    cactus_width, cactus.height,
                );

                // detect collision
                if dino_rect.overlaps(&cactus_rect) {
                    game_over = true;
                    break;
                }
            }
        }
        else {
            // restart the game
            if is_key_pressed(KeyCode::R) {
                dino_y = ground_y - dino_height;
                vel_y = 0.0;
                cacti = vec![
                    Cactus {
                        x: screen_width() * 0.8,
                        height: 50.0,
                    }
                ];
                score = 0.0;
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

        for cactus in cacti.iter() {
            let cactus_y = ground_y - cactus.height;

            // draw cactus
            draw_rectangle(
                cactus.x, cactus_y,
                cactus_width, cactus.height,
                BLACK
            );

            // draw cactus arms
            draw_rectangle(
                cactus.x - 8.0, cactus_y + cactus.height * 0.3,
                8.0, 6.0,
                BLACK,
            );

            draw_rectangle(
                cactus.x - 8.0, cactus_y + cactus.height * 0.1,
                6.0, cactus.height * 0.35,
                BLACK,
            );
        }

        // display score
        let score_text = format!("SCORE: {:04}", score as u32);

        draw_text(
            &score_text,
            screen_width() - 180.0,
            35.0, 25.0,
            BLACK,
        );

        // game instructions
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

fn spawn_cactus_pattern(cacti: &mut Vec<Cactus>, start_x: f32) {
    let pattern = macroquad::rand::gen_range(0, 3);

    match pattern {
        0 => {
            // one cactus
            cacti.push(Cactus {
                x: start_x,
                height: macroquad::rand::gen_range(35.0, 66.0),
            });
        }
        1 => {
            // two cacti
            for i in 0..2 {
                cacti.push(Cactus {
                    x: start_x + i as f32 * 35.0,
                    height: macroquad::rand::gen_range(35.0, 66.0),
                });
            }
        }
        _ => {
            // three cacti
            for i in 0..3 {
                cacti.push(Cactus {
                    x: start_x + i as f32 * 35.0,
                    height: macroquad::rand::gen_range(35.0, 66.0),
                });
            }
        }
    }
}