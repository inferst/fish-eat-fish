use std::collections::HashMap;

use macroquad::{
    math::Vec2,
    texture::{load_texture, Texture2D},
};

use crate::sprite::Options;

pub struct SpriteManager {
    sprites: HashMap<Texture, Options>,
}

impl SpriteManager {
    pub fn new() -> Self {
        let sprites: HashMap<Texture, Options> = HashMap::from([
            (
                Texture::Vobla,
                Options {
                    size: Vec2::new(578.0, 323.0),
                    offset: Vec2::new(32.0, 88.0),
                    collider: Vec2::new(500.0, 170.0),
                    frames: 1,
                    scale: 0.1,
                },
            ),
            //(
            //    Texture::Vobla,
            //    Options {
            //        size: Vec2::new(39.0, 20.0),
            //        offset: Vec2::new(2.0, 4.0),
            //        collider: Vec2::new(34.0, 13.0),
            //        frames: 4,
            //        scale: 1.0,
            //    },
            //),
            (
                Texture::FishDart,
                Options {
                    size: Vec2::new(39.0, 20.0),
                    offset: Vec2::new(2.0, 4.0),
                    collider: Vec2::new(34.0, 13.0),
                    frames: 4,
                    scale: 1.0,
                },
            ),
            (
                Texture::Fish,
                Options {
                    size: Vec2::new(32.0, 32.0),
                    offset: Vec2::new(2.0, 6.0),
                    collider: Vec2::new(26.0, 20.0),
                    frames: 4,
                    scale: 1.0,
                },
            ),
            (
                Texture::FishBig,
                Options {
                    size: Vec2::new(54.0, 49.0),
                    offset: Vec2::new(5.0, 4.0),
                    collider: Vec2::new(44.0, 39.0),
                    frames: 4,
                    scale: 1.0,
                },
            ),
            (
                Texture::Fish1,
                Options {
                    size: Vec2::new(29.0, 25.0),
                    offset: Vec2::new(0.0, 0.0),
                    collider: Vec2::new(29.0, 25.0),
                    frames: 1,
                    scale: 1.0,
                },
            ),
            (
                Texture::Fish3,
                Options {
                    size: Vec2::new(14.0, 11.0),
                    offset: Vec2::new(0.0, 0.0),
                    collider: Vec2::new(14.0, 11.0),
                    frames: 4,
                    scale: 1.0,
                },
            ),
            (
                Texture::Fish4,
                Options {
                    size: Vec2::new(14.0, 11.0),
                    offset: Vec2::new(0.0, 0.0),
                    collider: Vec2::new(14.0, 11.0),
                    frames: 4,
                    scale: 1.0,
                },
            ),
        ]);

        Self { sprites }
    }

    pub fn get_sprite_options(&self, name: &Texture) -> &Options {
        self.sprites.get(name).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Texture {
    Fish,
    FishDart,
    FishBig,
    Fish1,
    Fish3,
    Fish4,
    Vobla,
}

pub struct Manager {
    manifest: HashMap<Texture, String>,
    textures: HashMap<Texture, Texture2D>,
    sprites: SpriteManager,
}

impl Manager {
    pub fn new() -> Self {
        let textures = HashMap::new();

        let manifest: HashMap<Texture, String> = HashMap::from([
            (Texture::Vobla, String::from("assets/vobla.png")),
            //(Texture::Vobla, String::from("assets/fish-dart.png")),
            (Texture::Fish, String::from("assets/fish.png")),
            (Texture::FishDart, String::from("assets/fish-dart.png")),
            (Texture::FishBig, String::from("assets/fish-big.png")),
            (Texture::Fish1, String::from("assets/fish_1.png")),
            (Texture::Fish3, String::from("assets/fish_3.png")),
            (Texture::Fish4, String::from("assets/fish_4.png")),
        ]);

        let sprites = SpriteManager::new();

        Self {
            manifest,
            textures,
            sprites,
        }
    }

    pub async fn load(&mut self) {
        for (key, value) in &mut self.manifest {
            let texture = load_texture(value).await.unwrap();
            self.textures.insert(key.clone(), texture);
        }
    }

    pub fn get_texture(&self, name: &Texture) -> &Texture2D {
        self.textures.get(name).unwrap()
    }

    pub fn get_sprite(&self, name: &Texture) -> &Options {
        self.sprites.get_sprite_options(name)
    }
}
