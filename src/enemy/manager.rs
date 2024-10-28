use macroquad::prelude::*;

use crate::constants::ORIGINAL_SCREEN_WIDTH;

use super::enemy::Enemy;

#[derive(Default)]
pub struct Manager {
    pub enemies: Vec<Enemy>,
    spawn_timer: f32,
    textures: Vec<Texture2D>,
}

const MAX_ENEMIES: usize = 10;

impl Manager {
    pub async fn load_textures(&mut self) {
        let texture = load_texture("assets/fish-dart.png").await.unwrap();
        self.textures.push(texture);

        let texture = load_texture("assets/fish.png").await.unwrap();
        self.textures.push(texture);

        let texture = load_texture("assets/fish-big.png").await.unwrap();
        self.textures.push(texture);
    }

    pub fn restart(&mut self) {
        self.enemies.clear();
    }

    pub fn spawn(&mut self) {
        if self.spawn_timer <= 0.0 && self.enemies.len() < MAX_ENEMIES {
            let level = rand::gen_range(0, 3);
            let texture = &self.textures[level];
            let enemy = Enemy::new(texture.clone(), level);
            debug!("Enemy: {:?}", enemy);

            self.enemies.push(enemy);
            self.spawn_timer = rand::gen_range(0.0, 5.0);
        }
    }

    pub fn draw(&mut self) {
        for enemy in &mut self.enemies {
            let speed = if enemy.direction {
                enemy.speed
            } else {
                -enemy.speed
            };

            let x = speed * get_frame_time();

            enemy.collider.x += x;
            enemy.draw();
        }

        self.enemies.retain(|enemy| {
            enemy.collider.x >= -enemy.collider.w
                && enemy.collider.x <= ORIGINAL_SCREEN_WIDTH + enemy.collider.w
                && !enemy.is_dead
        });

        self.spawn_timer -= get_frame_time();
    }
}
