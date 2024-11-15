use crate::{enemy::Manager, player::Player};

pub const SCORE: [u32; 6] = [0, 50, 200, 500, 1000, 2000];

pub const ENEMY_SPEED_RANGE: (f32, f32) = (10.0, 50.0);

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
            if enemy.collider.overlaps(&player.get_collider()) {
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
