use macroquad::audio::{load_sound, play_sound, PlaySoundParams}; use macroquad::color::{YELLOW, WHITE, SKYBLUE}; use macroquad::input::{is_key_pressed, KeyCode}; use macroquad::prelude::{ draw_text, next_frame, clear_background, screen_width, screen_height, draw_circle, draw_rectangle, load_texture }; use macroquad::audio::stop_sound; use macroquad::texture::{DrawTextureParams, Texture2D}; use components::{Bird, Pipe, Background, CloudManager}; use rand::{thread_rng, Rng}; mod components;

const GRAVITY: f32 = 0.5; const JUMP_STRENGTH: f32 = -8.0; const PIPE_WIDTH: f32 = 50.0; const PIPE_GAP_INITIAL: f32 = 150.0; const PIPE_SPEED_INITIAL: f32 = 2.0; const BACKGROUND_SCROLL_SPEED: f32 = 1.0;

#[macroquad::main("Flappy Pombo")] async fn main() { let bg_texture = load_texture("src/BgImage.png") .await .expect("Falha ao carregar background");

Copy code
let jump_sound = load_sound("src/BirdJump.wav")
    .await
    .expect("Falha ao carregar som de pulo");
let death_sound = load_sound("src/BirdDeath.wav")
    .await
    .expect("Falha ao carregar som de morte");
let menu_music = load_sound("src/MainMenu-ThemeMusic.wav")
    .await
    .expect("Falha ao carregar música do menu");
let start_sound = load_sound("src/StartGame.wav")
    .await
    .expect("Falha ao carregar som de início");

let mut background = Background::new(bg_texture);
let mut clouds = CloudManager::new().await;
let mut bird = Bird {
    y: screen_height() / 2.0,
    velocity: 0.0,
};
let mut pipes: Vec<Pipe> = vec![Pipe {
    x: screen_width(),
    height: thread_rng().gen_range(50.0..300.0),
}];
let mut game_over = false;
let mut score = 0;
let mut pipe_speed = PIPE_SPEED_INITIAL;
let mut pipe_gap = PIPE_GAP_INITIAL;
let mut game_started = false;

play_sound(
    &menu_music,
    PlaySoundParams {
        looped: true,
        volume: 1.0,
    },
);

loop {
    clear_background(SKYBLUE);
    background.update(BACKGROUND_SCROLL_SPEED);
    background.draw();
    clouds.update();
    clouds.draw();

    if !game_started {
        draw_text(
            "FLAPPY POMBO",
            screen_width() / 4.0,
            screen_height() / 3.0,
            50.0,
            WHITE,
        );
        draw_text(
            "Pressione ESPAÇO para começar",
            screen_width() / 4.0,
            screen_height() / 2.0,
            30.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Space) {
            play_sound(
                &start_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
            stop_sound(&menu_music);
            game_started = true;
        }
    } else if game_over {
        draw_text(
            "Game Over! Press SPACE to restart",
            screen_width() / 4.0,
            screen_height() / 2.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Score: {}", score),
            screen_width() / 2.0 - 30.0,
            screen_height() / 2.0 + 40.0,
            30.0,
            WHITE,
        );
        
        if is_key_pressed(KeyCode::Space) {
            play_sound(
                &start_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
            bird = Bird {
                y: screen_height() / 2.0,
                velocity: 0.0,
            };
            pipes = vec![Pipe {
                x: screen_width(),
                height: thread_rng().gen_range(50.0..300.0),
            }];
            game_over = false;
            score = 0;
            pipe_speed = PIPE_SPEED_INITIAL;
            pipe_gap = PIPE_GAP_INITIAL;
        }
    } else {
        bird.update(&jump_sound);
        bird.draw();

        if bird.y > screen_height() {
            play_sound(
                &death_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
            game_over = true;
        }

        let mut should_remove = false;
        let mut collision = false;
        let mut new_pipe = None;

        if let Some(pipe) = pipes.first_mut() {
            pipe.update(pipe_speed);
            pipe.draw(pipe_gap);

            collision = 50.0 + 15.0 > pipe.x 
                && 50.0 - 15.0 < pipe.x + PIPE_WIDTH 
                && (bird.y - 15.0 < pipe.height 
                || bird.y + 15.0 > pipe.height + pipe_gap);

            should_remove = pipe.x < 50.0 - PIPE_WIDTH;

            if should_remove {
                new_pipe = Some(Pipe {
                    x: screen_width(),
                    height: thread_rng().gen_range(50.0..300.0),
                });
            }
        }

        if collision {
            play_sound(
                &death_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 1.0,
                },
            );
            game_over = true;
        }

        if should_remove {
            score += 1;
            pipes.remove(0);
            if let Some(pipe) = new_pipe {
                pipes.push(pipe);
            }

            if score % 5 == 0 {
                pipe_speed += 0.5;
                pipe_gap -= 5.0;
            }
        }
    }

    next_frame().await;
}
}