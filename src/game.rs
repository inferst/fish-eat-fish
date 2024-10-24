use crate::{enemy, player::Player};

pub struct Game {
    pub is_game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            is_game_over: false,
        }
    }

    pub fn check_for_collisions(&mut self, player: &Player, enemy_manager: &enemy::Manager) {
        if enemy_manager.collide(player.collider) {
            self.is_game_over = true;
        }
    }
}
