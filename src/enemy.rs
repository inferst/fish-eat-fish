use macroquad::{prelude::*, rand};

use crate::constants::{LEVELS, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug)]
pub struct Enemy {
    pub collider: Rect,
    pub speed: f32,
    pub direction: bool,
    pub level: u8, // 1,2,3,4,5
}

impl Enemy {
    pub fn create() -> Enemy {
        let direction = rand::gen_range(0, 2) == 0;
        let speed = rand::gen_range(100.0, 300.0);
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let level = rand::gen_range(0, 4) as u8;
        let size = LEVELS[level as usize];
        let x = if direction { -size.x } else { SCREEN_WIDTH };
        let y = rand::gen_range(-size.y / 2.0, SCREEN_HEIGHT - size.y / 2.0);

        Enemy {
            collider: Rect::new(x, y, size.x, size.y),
            speed,
            direction,
            level,
        }
    }
}

#[derive(Default)]
pub struct Manager {
    enemies: Vec<Enemy>,
    spawn_timer: f32,
}

impl Manager {
    pub fn draw(&mut self) {
        if self.spawn_timer <= 0.0 {
            let enemy = Enemy::create();
            debug!("Enemy: {:?}", enemy);

            self.enemies.push(enemy);
            self.spawn_timer = rand::gen_range(0.0, 2.0);
        }

        for enemy in &mut self.enemies {
            let speed = if enemy.direction {
                enemy.speed
            } else {
                -enemy.speed
            };

            enemy.collider.x += speed * get_frame_time();

            draw_rectangle(
                enemy.collider.x,
                enemy.collider.y,
                enemy.collider.w,
                enemy.collider.h,
                RED,
            );
        }

        self.enemies.retain(|enemy| {
            enemy.collider.x >= -enemy.collider.w
                && enemy.collider.x <= SCREEN_WIDTH + enemy.collider.w
        });

        self.spawn_timer -= get_frame_time();
    }

    pub fn collide(&self, collider: Rect) -> bool {
        self.enemies
            .iter()
            .any(|enemy| enemy.collider.overlaps(&collider))
    }
}
