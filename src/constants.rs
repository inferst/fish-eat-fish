use macroquad::math::Vec2;

pub const LEVELS: [Vec2; 5] = [
    Vec2::new(60.0, 40.0),
    Vec2::new(80.0, 50.0),
    Vec2::new(100.0, 60.0),
    Vec2::new(120.0, 70.0),
    Vec2::new(140.0, 80.0),
];

pub const SCREEN_SCALE: f32 = 3.0;

pub const ORIGINAL_SCREEN_WIDTH: f32 = 320.0;
pub const ORIGINAL_SCREEN_HEIGHT: f32 = 180.0;

pub const SCREEN_WIDTH: f32 = ORIGINAL_SCREEN_WIDTH * SCREEN_SCALE;
pub const SCREEN_HEIGHT: f32 = ORIGINAL_SCREEN_HEIGHT * SCREEN_SCALE;
