pub const SCORE: [u32; 3] = [0, 50, 200];

pub const WEIGHT: [u32; 3] = [10, 30, 50];

pub const ENEMY_SPEED_RANGE: [f32; 2] = [10.0, 50.0];

pub const FISH_SPRITES_OPTIONS: [ActorSpriteOptions; 3] = [
    ActorSpriteOptions {
        size: Vec2::new(39.0, 20.0),
        offset: Vec2::new(2.0, 4.0),
        collider: Vec2::new(34.0, 13.0),
        frames: 4,
    },
    ActorSpriteOptions {
        size: Vec2::new(32.0, 32.0),
        offset: Vec2::new(2.0, 6.0),
        collider: Vec2::new(26.0, 20.0),
        frames: 4,
    },
    ActorSpriteOptions {
        size: Vec2::new(54.0, 49.0),
        offset: Vec2::new(5.0, 4.0),
        collider: Vec2::new(44.0, 39.0),
        frames: 4,
    },
];

use macroquad::math::Vec2;

use crate::{actor::ActorSpriteOptions, enemy::Manager, player::Player};

pub struct Game {
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Game { game_over: false }
    }

    pub fn restart(&mut self) {
        self.game_over = false;
    }

    pub fn check_for_collisions(&mut self, player: &mut Player, enemy_manager: &mut Manager) {
        for enemy in &mut enemy_manager.enemies {
            if enemy.collider.overlaps(&player.collider) {
                if player.level >= enemy.level {
                    player.eat(enemy);
                    enemy.is_dead = true;
                } else {
                    self.game_over = true;
                }
            }
        }
    }
}
