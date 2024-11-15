use macroquad::prelude::*;

use crate::constants::{ORIGINAL_SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn game_screen_width() -> f32 {
    screen_width()
}

pub fn game_screen_height() -> f32 {
    screen_height()
}

pub fn screen_scale() -> f32 {
    screen_width() / ORIGINAL_SCREEN_WIDTH
}

pub fn conf() -> Conf {
    #[allow(clippy::cast_possible_truncation)]
    Conf {
        window_title: String::from("Fish and Chips"),
        fullscreen: true,
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}
