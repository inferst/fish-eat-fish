use animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use std::f32::consts::FRAC_1_SQRT_2;

use crate::{
    assets::{self, Texture},
    constants::{ORIGINAL_SCREEN_HEIGHT, ORIGINAL_SCREEN_WIDTH},
    enemy::Enemy,
    game::SCORE,
    sprite::{Options, Sprite},
    window::screen_scale,
};

pub struct Player {
    pub collider: Rect,
    speed: f32,
    direction: bool,
    pub level: u8,
    pub weight: u32,
    sprite: Sprite,
}

impl Player {
    pub fn new(options: &Options) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let sprite = AnimatedSprite::new(
            options.size.x as u32,
            options.size.y as u32,
            &[
                Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: u32::from(options.frames),
                    fps: 1,
                },
                Animation {
                    name: "swim".to_string(),
                    row: 0,
                    frames: u32::from(options.frames),
                    fps: 3,
                },
            ],
            true,
        );

        Self {
            collider: Rect::default(),
            level: 0,
            weight: 0,
            speed: 0.0,
            direction: true,
            sprite: Sprite::new(sprite, *options),
        }
    }

    pub fn reset(&mut self) {
        self.speed = 100.0;
        self.level = 0;
        self.weight = 0;
        self.collider = Rect::new(
            ORIGINAL_SCREEN_WIDTH / 2.0 - self.sprite.options.size.x / 2.0,
            ORIGINAL_SCREEN_HEIGHT / 2.0 - self.sprite.options.size.y / 2.0,
            self.sprite.options.size.x,
            self.sprite.options.size.y,
        );
    }

    pub fn eat(&mut self, enemy: &Enemy) {
        self.weight += enemy.weight;

        #[allow(clippy::cast_possible_truncation)]
        for (level, weight) in SCORE.iter().enumerate() {
            if self.weight >= *weight {
                self.level = level as u8;
            } else {
                break;
            }
        }
    }

    pub fn draw(&mut self, assets: &assets::Manager) {
        let options = self.sprite.options;

        self.collider.x -= (options.collider.x - self.collider.w) / 2.0;
        self.collider.y -= (options.collider.y - self.collider.h) / 2.0;

        self.collider.w = options.collider.x;
        self.collider.h = options.collider.y;

        let mut velocity = Vec2::default();

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            velocity.y -= self.speed * get_frame_time();
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            velocity.y += self.speed * get_frame_time();
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            velocity.x += self.speed * get_frame_time();
            self.direction = true;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            velocity.x -= self.speed * get_frame_time();
            self.direction = false;
        }

        if velocity.x != 0.0 && velocity.y != 0.0 {
            velocity.x *= FRAC_1_SQRT_2;
            velocity.y *= FRAC_1_SQRT_2;
        }

        let mut player_position =
            Vec2::new(self.collider.x + velocity.x, self.collider.y + velocity.y);

        player_position.x = player_position
            .x
            .clamp(0.0, ORIGINAL_SCREEN_WIDTH - self.collider.w);

        player_position.y = player_position
            .y
            .clamp(0.0, ORIGINAL_SCREEN_HEIGHT - self.collider.h);

        self.collider.move_to(player_position);

        let factor = self.factor();

        let texture = assets.get_texture(&Texture::Vobla);
        let sprite = &mut self.sprite;

        if velocity.x != 0.0 || velocity.y != 0.0 {
            sprite.sprite.set_animation(1);
        } else {
            sprite.sprite.set_animation(0);
        }

        let mut dest_size = sprite.sprite.frame().dest_size;
        dest_size.x *= screen_scale() * options.scale;
        dest_size.y *= screen_scale() * options.scale;

        draw_texture_ex(
            texture,
            (self.collider.x - sprite.options.offset.x) * screen_scale(),
            (self.collider.y - sprite.options.offset.y) * screen_scale(),
            RED,
            DrawTextureParams {
                source: Some(sprite.sprite.frame().source_rect),
                dest_size: Some(dest_size),
                flip_x: !self.direction,
                ..Default::default()
            },
        );

        sprite.sprite.update();
    }

    pub fn factor(&self) -> f32 {
        //0.001 * f32::from(self.level + 1)
        //0.2
        1.0
    }

    pub fn get_collider(&self) -> Rect {
        let mut collider = self.collider;
        collider.scale(self.factor(), self.factor());
        collider
    }
}
