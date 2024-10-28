use background::Background;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use enemy::Manager;
use game::Game;
use macroquad::prelude::*;
use player::Player;

mod actor;
mod background;
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

    set_default_filter_mode(FilterMode::Nearest);

    let mut background = Background::new();
    background.load_textures().await;

    let mut player = Player::new();
    player.load_texure().await;
    player.reset();

    let mut enemy_manager = Manager::default();
    enemy_manager.load_textures().await;

    let mut game = Game::new();

    loop {
        background.draw();

        enemy_manager.spawn();
        enemy_manager.draw();

        draw_text(player.level.to_string().as_str(), 10.0, 20.0, 40.0, WHITE);
        draw_text(player.weight.to_string().as_str(), 10.0, 60.0, 40.0, WHITE);

        if is_key_pressed(KeyCode::R) {
            player.reset();
            enemy_manager.restart();
            game.restart();
            next_frame().await;
            continue;
        }

        if game.game_over {
            next_frame().await;
            continue;
        }

        player.draw();

        game.check_for_collisions(&mut player, &mut enemy_manager);

        next_frame().await;
    }
}
