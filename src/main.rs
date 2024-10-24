use background::Background;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use enemy::Manager;
use game::Game;
use macroquad::prelude::*;
use player::Player;

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

    let mut player = Player::create();
    player.load_texure().await;

    let mut enemy_manager = Manager::default();

    let mut game = Game::new();

    loop {
        background.draw();

        enemy_manager.draw();

        if game.is_game_over {
            next_frame().await;
            continue;
        }

        player.draw();

        game.check_for_collisions(&player, &enemy_manager);

        next_frame().await;
    }
}
