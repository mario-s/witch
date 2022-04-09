extern crate ai_behavior;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::input::*;
use sprite::*;

use game::assets::Assets;
use game::background::Background;
use game::controller::Controller;
use game::figures::*;

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
    pub pause: bool,
}

impl Canvas {
    pub fn new(opengl: OpenGL) -> Canvas {
        let player = Player::new();
        let opponent = Opponent::new();
        let scenes = [player.as_scene(), opponent.as_scene()];

        let bg = Background::new();
        let bg_dim = bg.get_dimension();

        let controller =
            Controller::new(bg_dim, [(bg_dim[0] / 2.0) - 50.0, bg_dim[1] / 2.0], player);

        Canvas {
            gl: GlGraphics::new(opengl),
            background: bg,
            controller,
            font_path: Assets::assets("FreeSans.ttf"),
            scenes,
            pause: true,
        }
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, r_arg: RenderArgs) {
        let x_shifts = self.background.x_shifts;
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

            for texture in imgs.iter() {
                //append two images for a continues scrolling background
                image(texture, mat.trans(x_shifts[index], 0.0), g);
                image(texture, mat.trans(x_shifts[index] + width, 0.0), g);
                index += 1;
            }

            if pause {
                text(
                    BLACK,
                    40,
                    TEXT,
                    &mut cache,
                    mat.trans(width / 2.0 + 30.0, height / 2.0),
                    g,
                );
            }

            if !pause {
                let loc = ctrl.opponent_location;
                opponent.draw(mat.trans(loc[0], loc[1]), g);
            }

            let loc = ctrl.player_location;
            player.draw(mat.trans(loc[0], loc[1]), g);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.background.update();
        if !self.pause {
            self.controller.time_event(args.dt);
        }
    }

    pub fn toggle(&mut self, b: Button) {
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
