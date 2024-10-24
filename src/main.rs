use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use enemy::Manager;
use game::Game;
use macroquad::prelude::*;
use player::Player;

mod constants;
mod enemy;
mod game;
mod player;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fish and Chips".to_owned(),
        fullscreen: false,
        #[allow(clippy::cast_possible_truncation)]
        window_width: SCREEN_WIDTH as i32,
        #[allow(clippy::cast_possible_truncation)]
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn setup_debug() {
    miniquad::window::set_window_position(500, 300);
}

#[macroquad::main(window_conf)]
async fn main() {
    setup_debug();

    let background = load_texture("assets/background.png").await.unwrap();

    let mut player = Player::create();
    player.load_texure().await;

    let mut enemy_updater = Manager::default();

    let mut game = Game::new();

    loop {
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            Color::from_hex(0x00ff_ffff),
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: SCREEN_WIDTH,
                    y: SCREEN_HEIGHT,
                }),
                ..Default::default()
            },
        );

        enemy_updater.update();

        if game.is_game_over {
            next_frame().await;
            continue;
        }

        player.update();

        game.check_for_collisions(&player, &enemy_updater);

        next_frame().await;
    }
}
