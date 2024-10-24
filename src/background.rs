use std::ops::Mul;

use macroquad::prelude::*;

use crate::constants::{SCREEN_HEIGHT, SCREEN_SCALE, SCREEN_WIDTH};

pub struct Background {
    far_texture: Option<Texture2D>,
    sand_texture: Option<Texture2D>,
    foreground_texture: Option<Texture2D>,
}

impl Background {
    pub fn new() -> Self {
        Background {
            far_texture: None,
            sand_texture: None,
            foreground_texture: None,
        }
    }

    pub async fn load_textures(&mut self) {
        let far_texture = load_texture("assets/background/far.png").await.unwrap();
        self.far_texture = Some(far_texture);

        let sand_texture = load_texture("assets/background/sand.png").await.unwrap();
        self.sand_texture = Some(sand_texture);

        let foreground_texture = load_texture("assets/background/foreground-merged.png")
            .await
            .unwrap();
        self.foreground_texture = Some(foreground_texture);
    }

    fn draw_repeated_texture(texture: &Texture2D, offset: Vec2) {
        let width = texture.width() * SCREEN_SCALE;
        let height = texture.height() * SCREEN_SCALE;

        let columns = (SCREEN_WIDTH / width).ceil() as u32;
        let rows = (SCREEN_HEIGHT / height).ceil() as u32;

        //debug!("Column: {}", columns);
        //debug!("Rows: {}", rows);

        for column in 0..columns {
            for row in 0..rows {
                draw_texture_ex(
                    texture,
                    width.mul(column as f32) + offset.x * SCREEN_SCALE,
                    height.mul(row as f32) + offset.y * SCREEN_SCALE,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2 {
                            x: width,
                            y: height,
                        }),
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn draw(&self) {
        let far_texture = self.far_texture.as_ref().unwrap();
        Self::draw_repeated_texture(far_texture, Vec2::default());

        let frequency = 0.1;
        let amplitude_x = 5.0;
        let amplitude_y = 2.5;

        let offset_x = f64::cos(get_time() * frequency) as f32 * amplitude_x - amplitude_x;
        let offset_y = f64::sin(get_time() * frequency) as f32 * amplitude_y + amplitude_y;

        let sand_texture = self.sand_texture.as_ref().unwrap();
        Self::draw_repeated_texture(sand_texture, Vec2::new(offset_x, offset_y - 12.0));

        let frequency = 0.1;
        let amplitude_x = 10.0;
        let amplitude_y = 5.0;

        let offset_x = f64::cos(get_time() * frequency) as f32 * amplitude_x - amplitude_x;
        let offset_y = f64::sin(get_time() * frequency) as f32 * amplitude_y + amplitude_y;

        let foreground_texture = self.foreground_texture.as_ref().unwrap();
        Self::draw_repeated_texture(foreground_texture, Vec2::new(offset_x, offset_y - 12.0));
    }
}
