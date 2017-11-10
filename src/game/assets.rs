extern crate find_folder;

use std::path::PathBuf;

pub struct Assets {}

impl Assets {
    pub fn assets(path: &str) -> PathBuf {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        return assets.join(path);
    }
}