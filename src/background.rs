use macroquad::prelude::*;

use crate::window::{game_screen_height, game_screen_width, screen_scale};

pub struct Background {
    far_texture: Option<Texture2D>,
    sand: Option<Texture2D>,
    foreground: Option<Texture2D>,
}

impl Background {
    pub fn new() -> Self {
        Background {
            far_texture: None,
            sand: None,
            foreground: None,
        }
    }

    pub async fn load_textures(&mut self) {
        let far_texture = load_texture("assets/background/far.png").await.unwrap();
        self.far_texture = Some(far_texture);

        let sand_texture = load_texture("assets/background/sand.png").await.unwrap();
        self.sand = Some(sand_texture);

        let foreground_texture = load_texture("assets/background/foreground-merged.png")
            .await
            .unwrap();
        self.foreground = Some(foreground_texture);
    }

    fn draw_repeated_texture(texture: &Texture2D, offset: Vec2) {
        let width = texture.width() * screen_scale();
        let height = texture.height() * screen_scale();

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let columns = (game_screen_width() / width).ceil() as u32;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let rows = (game_screen_height() / height).ceil() as u32;

        for column in 0..columns {
            for row in 0..rows {
                #[allow(clippy::cast_precision_loss)]
                let x = width * column as f32;

                #[allow(clippy::cast_precision_loss)]
                let y = height * row as f32;

                draw_texture_ex(
                    texture,
                    x + offset.x.round() * screen_scale(),
                    y + offset.y.round() * screen_scale(),
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

        #[allow(clippy::cast_possible_truncation)]
        let time = get_time() as f32;

        let offset_x = f32::cos(time * frequency) * amplitude_x - amplitude_x;
        let offset_y = f32::sin(time * frequency) * amplitude_y + amplitude_y;

        let sand_texture = self.sand.as_ref().unwrap();
        Self::draw_repeated_texture(sand_texture, Vec2::new(offset_x, offset_y - 12.0));

        let frequency = 0.2;
        let amplitude_x = 10.0;
        let amplitude_y = 5.0;

        let offset_x = f32::cos(time * frequency) * amplitude_x - amplitude_x;
        let offset_y = f32::sin(time * frequency) * amplitude_y + amplitude_y;

        let foreground_texture = self.foreground.as_ref().unwrap();
        Self::draw_repeated_texture(foreground_texture, Vec2::new(offset_x, offset_y - 12.0));
    }
}
