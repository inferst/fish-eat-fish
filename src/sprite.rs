use core::fmt;

use animation::AnimatedSprite;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub size: Vec2,
    pub offset: Vec2,
    pub collider: Vec2,
    pub frames: u8,
    pub scale: f32,
}

pub struct Sprite {
    pub sprite: AnimatedSprite,
    pub options: Options,
}

impl Sprite {
    pub fn new(sprite: AnimatedSprite, options: Options) -> Self {
        Self { sprite, options }
    }
}

impl fmt::Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ActorSprite: {:?}", self.options)
    }
}
