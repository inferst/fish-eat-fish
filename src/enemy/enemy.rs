use animation::{AnimatedSprite, Animation};
use macroquad::{prelude::*, rand};

use crate::{
    actor::ActorSprite,
    constants::{ORIGINAL_SCREEN_HEIGHT, ORIGINAL_SCREEN_WIDTH, SCREEN_SCALE},
    game::{ENEMY_SPEED_RANGE, FISH_SPRITES_OPTIONS, WEIGHT},
};

#[derive(Debug)]
pub struct Enemy {
    pub collider: Rect,
    pub speed: f32,
    pub direction: bool,
    pub level: u8,
    pub weight: u32,
    pub is_dead: bool,
    texture: Option<Texture2D>,
    sprite: ActorSprite,
}

impl Enemy {
    pub fn new(texture: Texture2D, level: usize) -> Enemy {
        let direction = rand::gen_range(0, 2) == 0;
        let speed = rand::gen_range(ENEMY_SPEED_RANGE[0], ENEMY_SPEED_RANGE[1]);
        let sprite = FISH_SPRITES_OPTIONS[level];
        let size = sprite.collider;
        let weight = WEIGHT[level];
        let x = if direction {
            -size.x
        } else {
            ORIGINAL_SCREEN_WIDTH
        };
        let y = rand::gen_range(-size.y / 2.0, ORIGINAL_SCREEN_HEIGHT - size.y / 2.0);

        let options = FISH_SPRITES_OPTIONS[level];
        let sprite = AnimatedSprite::new(
            options.size.x as u32,
            options.size.y as u32,
            &[Animation {
                name: "".to_string(),
                row: 0,
                frames: options.frames as u32,
                fps: (speed / 10.0) as u32,
            }],
            true,
        );

        let level = level as u8;

        Enemy {
            collider: Rect::new(x, y, size.x, size.y),
            speed,
            direction,
            level,
            weight,
            is_dead: false,
            sprite: ActorSprite::new(sprite, options),
            texture: Some(texture),
        }
    }

    pub fn draw(&mut self) {
        let texture = self.texture.as_ref().unwrap();
        let sprite = &mut self.sprite.sprite;

        let mut dest_size = sprite.frame().dest_size;
        dest_size.x *= SCREEN_SCALE;
        dest_size.y *= SCREEN_SCALE;

        draw_texture_ex(
            texture,
            (self.collider.x - self.sprite.options.offset.x) * SCREEN_SCALE,
            (self.collider.y - self.sprite.options.offset.y) * SCREEN_SCALE,
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
