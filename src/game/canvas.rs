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


const WHITE: [f32; 4] = [1.0; 4];
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
        let w = bg.get_width();
        let h = bg.get_height();
        let controller = Controller::new(w/2.0, h/2.0, w, h);
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
        let translations = self.background.translations;
        let imgs = &self.background.levels;
        let width = imgs[0].get_width() as f64;
        let height = imgs[0].get_height() as f64;

        let mut cache = GlyphCache::new(Assets::assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();
        let controller = &self.controller;
        let pause = self.pause;
        let mut index = 0;

        let mut scene = Scene::new();
        scene.add_child(self.witch.sprite_at(0.0, 0.0));

        self.gl.draw(r_arg.viewport(), |c, g| {
            clear(WHITE, g);
            let mat = c.transform;

            for texture in imgs.into_iter() {
                let t = translations[index];
                //append two images for a continues scrolling background
                image(texture, mat.trans(t, 0.0), g);
                image(texture, mat.trans(t + width, 0.0), g);
                index += 1;
            }

            if pause {
                text(BLACK, 30, TEXT, &mut cache, 
                    mat.trans(width/2.0 + 30.0, height/2.0), g);
            }

            scene.draw(mat.trans(controller.horizontal, controller.vertical), g);
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