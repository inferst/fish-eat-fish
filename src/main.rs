use background::Background;
use enemy::Manager;
use game::Game;
use macroquad::prelude::*;
use player::Player;
use window::{conf, screen_scale};

mod assets;
mod background;
mod constants;
mod enemy;
mod game;
mod player;
mod sprite;
mod window;

fn setup_debug() {
    //miniquad::window::set_window_position(500, 300);
}

#[macroquad::main(conf)]
async fn main() {
    setup_debug();

    set_default_filter_mode(FilterMode::Nearest);

    let mut assets = assets::Manager::new();
    assets.load().await;

    let mut background = Background::new();
    background.load_textures().await;

    let mut player = Player::new(assets.get_sprite(&assets::Texture::FishDart));
    player.reset();

    let mut enemy_manager = Manager::default();

    let mut game = Game::new();

    let font = load_ttf_font("assets/04B_30__.TTF").await.unwrap();

    loop {
        background.draw();

        enemy_manager.draw(&assets, &player);

        draw_text_ex(
            &format!("SCORE {}", player.weight.to_string().as_str()),
            5.0 * screen_scale(),
            14.0 * screen_scale(),
            TextParams {
                font: Some(&font),
                font_size: 40,
                font_scale: screen_scale() / 4.0,
                ..Default::default()
            },
        );

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

        player.draw(&assets);

        game.check_for_collisions(&mut player, &mut enemy_manager);

        next_frame().await;
    }
}
