use std::rc::Rc;
use sprite::*;
use opengl_graphics::Texture;

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

    pub fn sprite_at(&mut self, x: f64, y: f64) -> Sprite<Texture> {
        let mut sprite = Sprite::from_texture(self.sprite.clone());
        sprite.set_position(x, y);
        return sprite;
    }
}
