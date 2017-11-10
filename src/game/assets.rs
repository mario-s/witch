extern crate find_folder;
extern crate opengl_graphics;

use std::path::PathBuf;
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
        let dir: &str = &path;
        return Texture::from_path(Assets::assets(dir), &TextureSettings::new()).unwrap();
    }
}