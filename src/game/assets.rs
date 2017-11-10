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