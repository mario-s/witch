extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;
extern crate ai_behavior;

use sprite::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::*;
use graphics::ImageSize;

use game::assets::*;
use game::controller::Controller;
use game::sprites::*;

use std::path::PathBuf;


const WHITE: [f32; 4] = [1.0; 4];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const TEXT: &str = "Witch Project";

/// The user interface which renders the game, characters, background, etc.
pub struct Canvas {
    gl: GlGraphics,
    background: Background,
    controller: Controller,
    font_path: PathBuf,
    scenes: [Scene<opengl_graphics::Texture>; 2],
    pub pause: bool
}

impl Canvas {
    pub fn new(opengl: OpenGL) -> Canvas {
        let mut player = Player::new();
        let mut opponent = Opponent::new();
        let player_width = player.get_width();
        let player_height = player.get_height();

        let mut bg = Background::new();
        let bg_w = bg.get_width();
        let bg_h = bg.get_height();

        let controller = Controller::new(player_width, player_height,
            (bg_w/2.0) - 50.0, bg_h/2.0,
            bg_w, bg_h);

        let font_path = Assets::assets("FreeSans.ttf");
        let mut scenes = [Scene::new(), Scene::new()];
        scenes[0].add_child(player.sprite());
        scenes[1].add_child(opponent.sprite());

        Canvas {
            gl: GlGraphics::new(opengl),
            background: bg,
            controller: controller,
            font_path,
            scenes: scenes,
            pause: true,
        }
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, r_arg: RenderArgs) {
        let translations = self.background.translations;
        let imgs = &self.background.levels;
        let width = imgs[0].get_width() as f64;
        let height = imgs[0].get_height() as f64;

        let mut cache = GlyphCache::new(&self.font_path, (), TextureSettings::new()).unwrap();
        let ctrl = &self.controller;
        let pause = self.pause;
        let mut index = 0;

        let player = &self.scenes[0];
        let opponent = &self.scenes[1];

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
                text(BLACK, 40, TEXT, &mut cache,
                    mat.trans(width/2.0 + 30.0, height/2.0), g);
            }

            if !pause {
                opponent.draw(mat.trans(width, height), g);
            }

            player.draw(mat.trans(ctrl.player_x, ctrl.player_y), g);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.background.animate();
        if !self.pause {
            self.controller.time_event(args.dt);
        }
    }

    pub fn toggle(&mut self,  b: Button) {
        if b == Button::Keyboard(Key::P) {
            self.pause = !self.pause;
        }
    }

    pub fn input(&mut self, s: ButtonState, k: Key) {
        if !self.pause {
            self.controller.key_event(s, k);
        }
    }
}