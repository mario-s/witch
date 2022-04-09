use graphics::ImageSize;
use opengl_graphics::Texture;
use sprite::*;
use std::rc::Rc;

use game::assets::*;

/// The user's player.
pub struct Player {
    sprite: Rc<Texture>,
}

/// The opponent of the player.
pub struct Opponent {
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
    pub fn new() -> Player {
        Player {
            sprite: Rc::new(Assets::icon("witch-icon.png")),
        }
    }
}

impl Figure for Player {
    fn image(&self) -> Rc<Texture> {
        self.sprite.clone()
    }
}

impl Opponent {
    pub fn new() -> Opponent {
        Opponent {
            sprite: Rc::new(Assets::icon("ape-44564.png")),
        }
    }
}

impl Figure for Opponent {
    fn image(&self) -> Rc<Texture> {
        self.sprite.clone()
    }
}
