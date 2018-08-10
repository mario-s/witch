extern crate find_folder;
extern crate opengl_graphics;

use std::path::PathBuf;
use graphics::ImageSize;
use opengl_graphics::{ TextureSettings, Texture};

pub struct Assets {}

impl Assets {
    pub fn assets(path: &str) -> PathBuf {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        return assets.join(path);
    }

    pub fn texture(name: &str) -> Texture {
        let path = "layers/".to_string() + name;
        return Assets::from_path(&path);
    }

    pub fn icon(name: &str) -> Texture {
        let path = "icons/".to_string() + name;
        return Assets::from_path(&path);
    }

    fn from_path(path: &str) -> Texture {
        return Texture::from_path(Assets::assets(path), &TextureSettings::new()).unwrap();
    }
}

pub struct Background {
    pub levels: [Texture; 4],
    pub translations: [f64; 4],
}

impl Background {
    pub fn new() -> Background {
        Background {
            levels: [
                Assets::texture("parallax-forest-back-trees.png"),
                Assets::texture("parallax-forest-middle-trees.png"),
                Assets::texture("parallax-forest-lights.png"),
                Assets::texture("parallax-forest-front-trees.png"),
            ],
            translations: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn get_width(&mut self) -> f64 {
        return self.levels[0].get_width() as f64;
    }

    pub fn animate(&mut self) {
        self.translations[0] -= 0.03;
        self.translations[1] -= 0.06;
        self.translations[3] -= 0.2;

        self.reset();
    }

    fn reset(&mut self) {
        let min: f64 = -1.0 * self.levels[0].get_width() as f64;
        for i in 0..4 {
            if self.translations[i] < min {
                self.translations[i] = 0.0;
            }
        }
    }
}