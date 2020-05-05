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
    pub levels: [Texture; 10],
    pub translations: [f64; 10],
}

impl Background {
    pub fn new() -> Background {
        Background {
            levels: [
                Assets::texture("10_Sky.png"),
                Assets::texture("09_Forest.png"),
                Assets::texture("08_Forest.png"),
                Assets::texture("07_Forest.png"),
                Assets::texture("06_Forest.png"),
                Assets::texture("05_Particles.png"),
                Assets::texture("04_Forest.png"),
                Assets::texture("03_Particles.png"),
                Assets::texture("02_Bushes.png"),
                Assets::texture("01_Mist.png"),
            ],
            translations: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn get_width(&mut self) -> f64 {
        self.levels[0].get_width() as f64
    }

    pub fn get_height(&mut self) -> f64 {
        self.levels[0].get_height() as f64
    }

    pub fn animate(&mut self) {
        let speed = 0.05;
        for i in 0..self.levels.len() {
            let f = (1 + 1 * i) as f64;
            self.translations[i] -= speed * f;
        }
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