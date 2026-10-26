use macroquad::prelude::*;

struct Cactus {
    x: f32,
    height: f32,
}

#[macroquad::main("dino game")]
async fn main() {
    let ground_y = 350.0;

    // load dino sprite sheet
    let dino_texture = load_texture("assets/dino_sheet.png")
        .await
        .expect("failed to load dino sprite sheet");
    
    dino_texture.set_filter(FilterMode::Nearest);

    // load mushroom sprite
    let mushroom_texture = load_texture("assets/mushroom.png")
        .await
        .expect("failed to load mushroom texture");

    mushroom_texture.set_filter(FilterMode::Nearest);

    // load misc textures
    let misc_texture = load_texture("assets/sprites_misc.png")
        .await
        .expect("failed to load misc textures");

    misc_texture.set_filter(FilterMode::Nearest);

    // load sky texture
    let sky_texture = load_texture("assets/sky.png")
        .await
        .expect("failed to load sky background");

    sky_texture.set_filter(FilterMode::Nearest);

    // load cloud texture
    let cloud_texture = load_texture("assets/clouds.png")
        .await
        .expect("failed to load cloud texture");

    cloud_texture.set_filter(FilterMode::Nearest);

    // load mountains texture
    let mountain_texture = load_texture("assets/mountains.png")
        .await
        .expect("failed to load mountain texture");

    mountain_texture.set_filter(FilterMode::Nearest);

    // dino properties
    let mut dino_y = ground_y - 80.0;
    let mut vel_y = 0.0;

    let dino_x = 100.0;
    let dino_width = 40.0;
    let dino_height = 80.0;

    let gravity = 1800.0;
    let jump_strength = -650.0;

    // dino animation
    let mut animation_timer = 0.0;
    let mut leg_frame = 0;
    let animation_speed = 0.12;

    // cactus properties
    let cactus_width = 44.0;
    let mut cacti = vec![
        Cactus {
            x: screen_width() * 0.8,
            height: 50.0,
        }
    ];
    let base_cactus_speed = 350.0;

    // mountain scrolling
    let mut mountain_offset = 0.0;
    let mountain_speed = 80.0;

    // clouds scrolling
    let mut cloud_offset = 0.0;
    let cloud_speed = 25.0;

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

            // animate legs only when on the ground
            if dino_y >= ground_y - dino_height {
                animation_timer += delta;

                if animation_timer >= animation_speed {
                    leg_frame = 1 - leg_frame;
                    animation_timer = 0.0;
                }
            }
            else {
                // reset animation while jumping
                leg_frame = 0;
                animation_timer = 0.0;
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

            // hitbox for collision detection
            let dino_rect = Rect::new(
                dino_x + 8.0, dino_y + 10.0,
                dino_width - 16.0, dino_height - 14.0,
            );

            for cactus in cacti.iter() {
                let cactus_rect = Rect::new(
                    cactus.x + 6.0, ground_y - cactus.height + 12.0,
                    cactus_width - 12.0, cactus.height - 12.0,
                );

                // detect collision
                if dino_rect.overlaps(&cactus_rect) {
                    game_over = true;
                    break;
                }
            }

            // scroll mountains
            mountain_offset += mountain_speed * delta;

            if mountain_offset >= mountain_texture.width() {
                mountain_offset -= mountain_texture.width();
            }

            // scroll clouds
            cloud_offset += cloud_speed * delta;

            if cloud_offset >= cloud_texture.width() {
                cloud_offset -= cloud_texture.width();
            }
        }
        else {
            // restart the game
            if is_key_pressed(KeyCode::R) {
                dino_y = ground_y - dino_height;
                vel_y = 0.0;
                animation_timer = 0.0;
                leg_frame = 0;
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

        // draw sky background
        clear_background(WHITE);

        draw_texture_ex(
            &sky_texture,
            0.0, 0.0,
            WHITE,

            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // draw clouds
        let cloud_width = cloud_texture.width();
        let mut cloud_x = -cloud_offset;

        while cloud_x < screen_width() {
            draw_texture_ex(
                &cloud_texture,
                cloud_x, 0.0,
                WHITE,

                DrawTextureParams {
                    dest_size: Some(vec2(cloud_texture.width(), cloud_texture.height())),
                    flip_y: true,
                    ..Default::default()
                },
            );

            cloud_x += cloud_width;
        }

        // draw mountains
        let mountain_width = mountain_texture.width();
        let mountain_height = mountain_texture.height();
        let mountain_y = ground_y - mountain_height;
        let mut mountain_x = -mountain_offset;

        while mountain_x < screen_width() {
            draw_texture(
                &mountain_texture,
                mountain_x, mountain_y,
                WHITE,
            );

            mountain_x += mountain_width;
        }

        // draw the ground
        let ground_width = 32.0;
        let ground_height = 32.0;
        let mut ground_x = 0.0;

        while ground_x < screen_width() {
            draw_texture_ex(
                &misc_texture,
                ground_x, ground_y,
                WHITE,

                DrawTextureParams {
                    source: Some(Rect::new(0.0, 0.0, 16.0, 16.0)),
                    dest_size: Some(vec2(ground_width, ground_height)),
                    ..Default::default()
                },
            );

            ground_x += ground_width;
        }

        // draw dino sprite with animation
        let sprite_x = if leg_frame == 0 {0.0} else {96.0};

        draw_texture_ex(
            &dino_texture,
            dino_x, dino_y + 24.0,
            WHITE,

            DrawTextureParams {
                source: Some(Rect::new(sprite_x, 40.0, 48.0, 56.0)),
                dest_size: Some(vec2(48.0, 56.0)),
                ..Default::default()
            },
        );

        // draw mushroom sprites
        for cactus in cacti.iter() {
            let mushroom_height = cactus.height;
            let mushroom_y = ground_y - mushroom_height + 6.0;

            draw_texture_ex(
                &mushroom_texture,
                cactus.x - 6.0, mushroom_y,
                WHITE,

                DrawTextureParams {
                    dest_size: Some(vec2(cactus_width, mushroom_height)),
                    ..Default::default()
                },
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
                screen_width() / 2.0 - 70.0,
                120.0, 40.0,
                RED,
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
                    x: start_x + i as f32 * 44.0,
                    height: macroquad::rand::gen_range(35.0, 66.0),
                });
            }
        }
        _ => {
            // three cacti
            for i in 0..3 {
                cacti.push(Cactus {
                    x: start_x + i as f32 * 44.0,
                    height: macroquad::rand::gen_range(35.0, 66.0),
                });
            }
        }
    }
}