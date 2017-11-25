extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;
extern crate ai_behavior;


use sprite::*;
use std::rc::Rc;
use piston::window::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings, Texture};
use graphics::*;

use game::assets::Assets;

struct Background {
    levels: [Texture; 4],
}

impl Background {
    fn new() -> Background {
        Background {
            levels: [
                Assets::texture("parallax-forest-back-trees.png"),
                Assets::texture("parallax-forest-middle-trees.png"),
                Assets::texture("parallax-forest-lights.png"),
                Assets::texture("parallax-forest-front-trees.png"),
            ]
        }
    }
}


pub struct Canvas {
    gl: GlGraphics,
    background: Background,
    translation: f64,
    background_translations: [f64; 4],
    witch: Rc<Texture>
}

impl Canvas {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const ZERO: f64 = 0.0;

    pub fn new(opengl: OpenGL) -> Canvas {
        Canvas {
            gl: GlGraphics::new(opengl),
            background: Background::new(),
            translation: 0.0,
            background_translations: [0.0, 0.0, 0.0, 0.0],
            witch: Rc::new(Assets::icon("witch-icon.png")),
        }
    }

    fn witch(&mut self) -> Sprite<Texture> {
        let mut sprite = Sprite::from_texture(self.witch.clone());
        sprite.set_position(50.0, 80.0);
        return sprite;
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, r_arg: RenderArgs) {
        let viewport = r_arg.viewport();

        let mut scene = Scene::new();
        scene.add_child(self.witch());

        let imgs = &self.background.levels;

        let mut cache = GlyphCache::new(Assets::assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();

        let translation = self.translation;
        let translations = self.background_translations;
        let mut index = 0;

        self.gl.draw(viewport, |context, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);

            for texture in imgs.into_iter() {
                let translation = translations[index];
                let width: f64 = texture.get_width() as f64;
                image(texture, context.transform.trans(translation, 0.0), gl);
                image(texture, context.transform.trans(width + translation, 0.0), gl);
                index += 1;
            }

            let trans = context.transform.trans(translation, 0.0);
            scene.draw(trans, gl);

            text(Canvas::BLACK, 30, &"Blair Witch", &mut cache, context.transform.trans(100.0, 90.0), gl);
        });
    }


    pub fn update(&mut self, args: UpdateArgs) {
        let max: f64 = self.background.levels[0].get_width() as f64;
        self.translation += 0.5;
        if self.translation > max {
            self.translation = Canvas::ZERO;
        }

        self.background_translations[0] -= 0.01;
        self.background_translations[1] -= 0.03;
        self.background_translations[3] -= 0.1;

        let min: f64 = -1.0 * max;
        for i in 0..4 {
            if self.background_translations[i] < min {
                self.background_translations[i] = Canvas::ZERO;
            }
        }
    }
}