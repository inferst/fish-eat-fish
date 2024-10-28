use core::fmt;

use animation::AnimatedSprite;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct ActorSpriteOptions {
    pub size: Vec2,
    pub offset: Vec2,
    pub collider: Vec2,
    pub frames: u8,
}

pub struct ActorSprite {
    pub sprite: AnimatedSprite,
    pub options: ActorSpriteOptions,
}

impl ActorSprite {
    pub fn new(sprite: AnimatedSprite, options: ActorSpriteOptions) -> Self {
        Self { sprite, options }
    }
}

impl fmt::Debug for ActorSprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ActorSprite: {:?}", self.options)
    }
}
