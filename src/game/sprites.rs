use std::rc::Rc;
use sprite::*;
use opengl_graphics::Texture;

use game::assets::*;

pub const WITCH_ICON: &str = "witch-icon.png";

//start position of the sprite for the witch
pub const WITCH_X: f64 = 50.0;
pub const WITCH_Y: f64 = 80.0;

pub struct Figure {
    sprite: Rc<Texture>,
}

impl Figure {

    pub fn new(icon: &str) -> Figure {
        Figure {
            sprite: Rc::new(Assets::icon(icon))
        }
    }

    pub fn sprite_at(&mut self, x: f64, y: f64) -> Sprite<Texture> {
        let mut sprite = Sprite::from_texture(self.sprite.clone());
        sprite.set_position(x, y);
        return sprite;
    }
}
