use std::rc::Rc;
use sprite::*;
use opengl_graphics::Texture;
use graphics::ImageSize;

use game::assets::*;

/// A figure is a character on the canvas. This can be either the player or opponent.
pub struct Player {
    sprite: Rc<Texture>,
}

pub struct Opponent {
    sprite: Rc<Texture>,
}

pub trait Icon {
    fn image(&self) -> Rc<Texture>;

    fn get_dimension(&self) -> [u32; 2] {
        [self.image().get_width(), self.image().get_height()]
    }

    fn sprite(&self) -> Sprite<Texture> {
        Sprite::from_texture(self.image())
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            sprite: Rc::new(Assets::icon("witch-icon.png"))
        }
    }
}

impl Icon for Player {

    fn image(&self) -> Rc<Texture> {
        self.sprite.clone()
    }
}

impl Opponent {
    pub fn new() -> Opponent {
        Opponent {
            sprite: Rc::new(Assets::icon("ape-44564.png"))
        }
    }
}

impl Icon for Opponent {

    fn image(&self) -> Rc<Texture> {
        self.sprite.clone()
    }
}
