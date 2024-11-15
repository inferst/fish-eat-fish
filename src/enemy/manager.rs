use macroquad::prelude::*;

use crate::{
    assets::{self, Texture},
    constants::ORIGINAL_SCREEN_WIDTH,
    player::Player,
};

use super::Enemy;

const MAX_ENEMIES: usize = 0;

struct EnemyOptions {
    texture: Texture,
    level: u8,
    weight: u32,
}

const ENEMIES: [EnemyOptions; 6] = [
    EnemyOptions {
        texture: Texture::Fish3,
        level: 0,
        weight: 5,
    },
    EnemyOptions {
        texture: Texture::Fish4,
        level: 0,
        weight: 5,
    },
    EnemyOptions {
        texture: Texture::FishDart,
        level: 1,
        weight: 10,
    },
    EnemyOptions {
        texture: Texture::Fish,
        level: 2,
        weight: 30,
    },
    EnemyOptions {
        texture: Texture::Fish1,
        level: 3,
        weight: 50,
    },
    EnemyOptions {
        texture: Texture::FishBig,
        level: 4,
        weight: 80,
    },
];

#[derive(Default)]
pub struct Manager {
    pub enemies: Vec<Enemy>,
    spawn_timer: f32,
}

impl Manager {
    pub fn restart(&mut self) {
        self.enemies.clear();
    }

    pub fn spawn(&mut self, assets: &assets::Manager, player: &Player) {
        if self.spawn_timer <= 0.0 && self.enemies.len() < MAX_ENEMIES {
            let length = u8::try_from(ENEMIES.len()).unwrap();
            let max_level = (player.level + 3).min(length);
            let level = rand::gen_range(0, max_level);
            let enemies: Vec<_> = ENEMIES
                .iter()
                .filter(|enemy| enemy.level == level)
                .collect();
            let enemy = enemies[rand::gen_range(0, enemies.len())];
            let options = assets.get_sprite(&enemy.texture);
            let enemy = Enemy::new(
                enemy.texture.clone(),
                *options,
                level as usize,
                enemy.weight,
            );

            self.enemies.push(enemy);
            self.spawn_timer = rand::gen_range(0.0, 3.0);
        }
    }

    pub fn draw(&mut self, assets: &assets::Manager, player: &Player) {
        self.spawn(assets, player);

        for enemy in &mut self.enemies {
            let speed = if enemy.direction {
                enemy.speed
            } else {
                -enemy.speed
            };

            let x = speed * get_frame_time();

            enemy.collider.x += x;
            enemy.draw(assets);
        }

        self.enemies.retain(|enemy| {
            enemy.collider.x >= -enemy.collider.w
                && enemy.collider.x <= ORIGINAL_SCREEN_WIDTH + enemy.collider.w
                && !enemy.is_dead
        });

        self.spawn_timer -= get_frame_time();
    }
}
