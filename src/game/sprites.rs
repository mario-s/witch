use std::rc::Rc;
use sprite::*;
use opengl_graphics::Texture;
use graphics::ImageSize;

use game::assets::*;

pub const WITCH_ICON: &str = "witch-icon.png";


pub struct Figure {
    sprite: Rc<Texture>,
}

impl Figure {

    pub fn new(icon: &str) -> Figure {
        Figure {
            sprite: Rc::new(Assets::icon(icon))
        }
    }

    pub fn sprite(&mut self) -> Sprite<Texture> {
        Sprite::from_texture(self.sprite.clone())
    }

    pub fn get_width(&mut self) -> u32 {
        self.sprite.get_width()
    }

    pub fn get_height(&mut self) -> u32 {
        self.sprite.get_height()
    }
}
