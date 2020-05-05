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
        assets.join(path)
    }

    pub fn texture(name: &str) -> Texture {
        let path = "layers/".to_string() + name;
        Assets::from_path(&path)
    }

    pub fn icon(name: &str) -> Texture {
        let path = "icons/".to_string() + name;
        Assets::from_path(&path)
    }

    fn from_path(path: &str) -> Texture {
        Texture::from_path(Assets::assets(path), &TextureSettings::new()).unwrap()
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
        self.levels[0].get_width() as f64
    }

    pub fn animate(&mut self) {
        let speed = 0.03;
        self.translations[0] -= speed;
        self.translations[1] -= speed * 2.0;
        self.translations[3] -= speed * 6.0;

        self.reset();
    }

    fn reset(&mut self) {
        let min: f64 = -1.0 * self.get_width();
        for i in 0..self.levels.len() {
            if self.translations[i] < min {
                self.translations[i] = 0.0;
            }
        }
    }
}