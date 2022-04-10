use graphics::{ImageSize, Transformed};
use opengl_graphics::{GlGraphics, Texture};
use sprite::*;
use std::rc::Rc;

use game::assets::*;

pub const WITCH: &str = "witch-icon.png";
pub const APE: &str = "ape-44564.png";

/// One player.
pub struct Player {
    texture: Rc<Texture>,
    sprite: Sprite<Texture>
}

/// This trait is for the capabilities of an player.
pub trait Figure {

    fn get_dimension(&self) -> [u32; 2] {
        [self.image().get_width(), self.image().get_height()]
    }

    fn image(&self) -> Rc<Texture>;

    fn draw_at(&self, loc: [f64; 2], mat: [[f64; 3]; 2], g: &mut GlGraphics) -> ();
}

impl Player {
    pub fn new(image_name: &str) -> Player {
        let texture = Rc::new(Assets::icon(image_name));
        let sprite = Sprite::from_texture(texture.clone());
        Player {
            texture,
            sprite
        }
    }
}

impl Figure for Player {
    fn image(&self) -> Rc<Texture> {
        self.texture.clone()
    }

    fn draw_at(&self, loc: [f64; 2], mat: [[f64; 3]; 2], g: &mut GlGraphics) -> () {
        self.sprite.draw(mat.trans(loc[0], loc[1]), g);
    }
}
