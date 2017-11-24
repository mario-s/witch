extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use sprite::*;
use std::rc::Rc;
use piston::window::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings, Texture};
use graphics::*;
use game::assets::Assets;

struct Env {
    backgrounds: [Texture; 4],
}

impl Env {
    fn new() -> Env {
        Env {
            backgrounds:  [
                Assets::texture("parallax-forest-back-trees.png"),
                Assets::texture("parallax-forest-middle-trees.png"),
                Assets::texture("parallax-forest-lights.png"),
                Assets::texture("parallax-forest-front-trees.png"),
            ]
        }
    }

    fn witch() -> Sprite<Texture> {
        let witch = Rc::new(Assets::icon("witch-icon.png"));
        let mut sprite = Sprite::from_texture(witch.clone());
        sprite.set_position(50.0, 80.0);
        return sprite;
    }
}


pub struct Canvas {
    gl: GlGraphics,
    env: Env,
    translation: f64,
    backgroundTranslations: [f64; 4],
}


impl Canvas {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const ZERO: f64 = 0.0;

    pub fn new(opengl: OpenGL) -> Canvas {
        Canvas{
            gl: GlGraphics::new(opengl),
            env: Env::new(),
            translation: 0.0,
            backgroundTranslations: [0.0,0.0,0.0,0.0],
        }
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, args: RenderArgs) {
        let viewport = args.viewport();

        let imgs = &self.env.backgrounds;

        let sprite = Env::witch();
        let mut scene = Scene::new();
        scene.add_child(sprite);

        let mut cache = GlyphCache::new(Assets::assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();

        let translation = self.translation;
        let translations = self.backgroundTranslations;
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
        self.translation += 0.5;

        self.backgroundTranslations[0] -= 0.01;
        self.backgroundTranslations[1] -= 0.02;
        self.backgroundTranslations[3] -= 0.04;

        let min: f64 = -1.0 * self.env.backgrounds[0].get_width() as f64;
        for i in 0..4 {
            if self.backgroundTranslations[i] < min {
                self.backgroundTranslations[i] = Canvas::ZERO;
            }
        }

    }

}