extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;
extern crate ai_behavior;

use sprite::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::*;

use game::assets::*;
use game::controller::Controller;
use game::sprites::*;


const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const TEXT: &str = "Blair Witch";

pub struct Canvas {
    gl: GlGraphics,
    background: Background,
    controller: Controller,
    witch: Figure,
    pub pause: bool
}

impl Canvas {
    pub fn new(opengl: OpenGL) -> Canvas {
        let mut bg = Background::new();
        let controller = Controller::new(bg.get_width(), bg.get_height());
        Canvas {
            gl: GlGraphics::new(opengl),
            background: bg,
            controller: controller,
            witch: Figure::new(WITCH_ICON),
            pause: true,
        }
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, r_arg: RenderArgs) {
        let viewport = r_arg.viewport();

        let mut scene = Scene::new();
        scene.add_child(self.witch.sprite_at(WITCH_X, WITCH_Y));

        let imgs = &self.background.levels;
        let mut cache = GlyphCache::new(Assets::assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();

        let horizontal = self.controller.horizontal;
        let vertical = self.controller.vertical;
        let translations = self.background.translations;
        let pause = self.pause;
        let mut index = 0;

        self.gl.draw(viewport, |c, g| {
            clear(WHITE, g);
            let mat = c.transform;

            for texture in imgs.into_iter() {
                let translation = translations[index];
                let width = texture.get_width() as f64;
                image(texture, mat.trans(translation, 0.0), g);
                image(texture, mat.trans(width + translation, 0.0), g);
                index += 1;
            }

            if pause {
                let img = &imgs[0];
                let w = img.get_width() as f64;
                let h = img.get_height() as f64;
                text(BLACK, 30, TEXT, &mut cache, mat.trans(w/2.0, h/2.0), g);
            }

            scene.draw(mat.trans(horizontal, vertical), g);
        });
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.background.animate();
    }

    pub fn input(&mut self, b: Button) {
        //println!("Pressed keyboard key '{:?}'", b);
        if !self.pause {
            self.controller.do_move(b);
        }
        
        self.toggle(b);
    }

    fn toggle(&mut self,  b: Button) {
        if b == Button::Keyboard(Key::P) {
            self.pause = !self.pause;
        }
    }
}