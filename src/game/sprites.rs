use std::rc::Rc;
use sprite::*;
use opengl_graphics::Texture;

use game::assets::*;

//start position of the sprite
pub const WITCH_START_X: f64 = 50.0;
pub const WITCH_START_Y: f64 = 80.0;

pub struct Witch {
    sprite: Rc<Texture>,
}

pub trait Figure {
    fn new() -> Witch {
        Witch {
            sprite: Rc::new(Assets::icon("witch-icon.png"))
        }
    }

    fn clone(&mut self) -> Sprite<Texture>;
}

impl Figure for Witch {

    fn clone(&mut self) -> Sprite<Texture> {
        let mut sprite = Sprite::from_texture(self.sprite.clone());
        sprite.set_position(WITCH_START_X, WITCH_START_Y);
        return sprite;
    }
}
