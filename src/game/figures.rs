use graphics::ImageSize;
use opengl_graphics::Texture;
use sprite::*;
use std::rc::Rc;

use game::assets::*;

pub const WITCH: &str = "witch-icon.png";
pub const APE: &str = "ape-44564.png";

/// One player.
pub struct Player {
    sprite: Rc<Texture>,
}

/// This trait is for the capabilities of an payer.
pub trait Figure {
    fn image(&self) -> Rc<Texture>;

    fn get_dimension(&self) -> [u32; 2] {
        [self.image().get_width(), self.image().get_height()]
    }

    fn as_scene(&self) -> Scene<opengl_graphics::Texture> {
        let mut scene = Scene::new();
        scene.add_child(self.sprite());
        scene
    }

    fn sprite(&self) -> Sprite<Texture> {
        Sprite::from_texture(self.image())
    }
}

impl Player {
    pub fn new(image_name: &str) -> Player {
        Player {
            sprite: Rc::new(Assets::icon(image_name)),
        }
    }
}

impl Figure for Player {
    fn image(&self) -> Rc<Texture> {
        self.sprite.clone()
    }
}
