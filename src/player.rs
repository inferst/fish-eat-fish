use macroquad::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;

use crate::constants::{LEVELS, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Player {
    pub collider: Rect,
    speed: f32,
    level: u8,
    texture: Option<Texture2D>,
}

impl Player {
    pub fn create() -> Self {
        let player_level = LEVELS[0];

        Player {
            collider: Rect::new(10.0, 10.0, player_level.x, player_level.y),
            level: 1,
            speed: 300.0,
            texture: None,
        }
    }

    pub async fn load_texure(&mut self) {
        self.texture = Some(load_texture("assets/player.png").await.unwrap());
    }

    pub fn update(&mut self) {
        let mut velocity = Vec2::default();

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            velocity.y -= self.speed * get_frame_time();
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            velocity.y += self.speed * get_frame_time();
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            velocity.x += self.speed * get_frame_time();
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            velocity.x -= self.speed * get_frame_time();
        }

        if velocity.x != 0.0 && velocity.y != 0.0 {
            velocity.x *= FRAC_1_SQRT_2;
            velocity.y *= FRAC_1_SQRT_2;
        }

        let mut player_position =
            Vec2::new(self.collider.x + velocity.x, self.collider.y + velocity.y);

        player_position.x = player_position.x.clamp(0.0, SCREEN_WIDTH - self.collider.w);

        player_position.y = player_position
            .y
            .clamp(0.0, SCREEN_HEIGHT - self.collider.h);

        self.collider.move_to(player_position);

        let texture = self.texture.as_ref().unwrap();

        draw_texture_ex(
            texture,
            self.collider.x,
            self.collider.y,
            Color::from_hex(0x00ff_ffff),
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: self.collider.w,
                    y: self.collider.h,
                }),
                ..Default::default()
            },
        );
    }
}
