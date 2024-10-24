use macroquad::{prelude::*, rand};

use crate::constants::{LEVELS, SCREEN_HEIGHT, SCREEN_WIDTH};

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
        let y = rand::gen_range(-size.x, SCREEN_HEIGHT);

        Enemy {
            collider: Rect::new(x, y, size.x, size.y),
            speed,
            direction,
            level,
        }
    }
}

#[derive(Default)]
pub struct Updater {
    enemies: Vec<Enemy>,
    spawn_timer: f32,
    spawn_timer_max: f32,
}

impl Updater {
    pub fn update(&mut self) {
        if self.spawn_timer > self.spawn_timer_max {
            let enemy = Enemy::create();
            self.enemies.push(enemy);
            self.spawn_timer = 0.0;
            self.spawn_timer_max = rand::gen_range(0.0, 2.0);
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

        self.spawn_timer += get_frame_time();
    }
}
