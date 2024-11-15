pub(crate) use manager::*;

mod manager;

use animation::{AnimatedSprite, Animation};
use macroquad::{prelude::*, rand};

use crate::{
    assets::{self, Texture},
    constants::{ORIGINAL_SCREEN_HEIGHT, ORIGINAL_SCREEN_WIDTH},
    game::ENEMY_SPEED_RANGE,
    sprite::{Options, Sprite},
    window::screen_scale,
};

#[derive(Debug)]
pub struct Enemy {
    pub collider: Rect,
    pub speed: f32,
    pub direction: bool,
    pub level: u8,
    pub weight: u32,
    pub is_dead: bool,
    texture: Texture,
    sprite: Sprite,
}

impl Enemy {
    pub fn new(texture: Texture, options: Options, level: usize, weight: u32) -> Enemy {
        let direction = rand::gen_range(0, 2) == 0;
        let speed = rand::gen_range(ENEMY_SPEED_RANGE.0, ENEMY_SPEED_RANGE.1);
        let size = options.collider;
        let x = if direction {
            -size.x
        } else {
            ORIGINAL_SCREEN_WIDTH
        };
        let y = rand::gen_range(-size.y / 2.0, ORIGINAL_SCREEN_HEIGHT - size.y / 2.0);

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let sprite = AnimatedSprite::new(
            options.size.x as u32,
            options.size.y as u32,
            &[Animation {
                name: String::new(),
                row: 0,
                frames: u32::from(options.frames),
                fps: (speed / 10.0) as u32,
            }],
            true,
        );

        let level = u8::try_from(level).unwrap();

        Enemy {
            collider: Rect::new(x, y, size.x, size.y),
            speed,
            direction,
            level,
            weight,
            is_dead: false,
            sprite: Sprite::new(sprite, options),
            texture,
        }
    }

    pub fn draw(&mut self, assets: &assets::Manager) {
        let texture = assets.get_texture(&self.texture);
        let sprite = &mut self.sprite.sprite;

        let mut dest_size = sprite.frame().dest_size;
        dest_size.x *= screen_scale();
        dest_size.y *= screen_scale();

        draw_texture_ex(
            texture,
            (self.collider.x - self.sprite.options.offset.x) * screen_scale(),
            (self.collider.y - self.sprite.options.offset.y) * screen_scale(),
            WHITE,
            DrawTextureParams {
                source: Some(sprite.frame().source_rect),
                dest_size: Some(dest_size),
                flip_x: !self.direction,
                ..Default::default()
            },
        );

        sprite.update();
    }
}
