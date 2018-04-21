extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;
extern crate ai_behavior;


use sprite::*;
use std::rc::Rc;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings, Texture};
use graphics::*;

use game::assets::Assets;

struct Background {
    levels: [Texture; 4],
    translations: [f64; 4],
}

impl Background {
    fn new() -> Background {
        Background {
            levels: [
                Assets::texture("parallax-forest-back-trees.png"),
                Assets::texture("parallax-forest-middle-trees.png"),
                Assets::texture("parallax-forest-lights.png"),
                Assets::texture("parallax-forest-front-trees.png"),
            ],
            translations: [0.0, 0.0, 0.0, 0.0],
        }
    }

    fn animate(&mut self) {
        self.translations[0] -= 0.01;
        self.translations[1] -= 0.03;
        self.translations[3] -= 0.1;

        let min: f64 = -1.0 * self.levels[0].get_width() as f64;
        for i in 0..4 {
            if self.translations[i] < min {
                self.translations[i] = 0.0;
            }
        }
    }
}


pub struct Canvas {
    gl: GlGraphics,
    background: Background,
    horizontal: f64,
    vertical: f64,
    witch: Rc<Texture>
}

impl Canvas {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const W_SPEED: f64 = 2.0;

    pub fn new(opengl: OpenGL) -> Canvas {
        Canvas {
            gl: GlGraphics::new(opengl),
            background: Background::new(),
            horizontal: 0.0,
            vertical: 0.0,
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

        let horizontal = self.horizontal;
        let vertical = self.vertical;    
        let translations = self.background.translations;
        let mut index = 0;

        self.gl.draw(viewport, |context, gl| {
            clear(Canvas::WHITE, gl);

            for texture in imgs.into_iter() {
                let translation = translations[index];
                let width: f64 = texture.get_width() as f64;
                image(texture, context.transform.trans(translation, 0.0), gl);
                image(texture, context.transform.trans(width + translation, 0.0), gl);
                index += 1;
            }

            let trans = context.transform.trans(horizontal, vertical);
            scene.draw(trans, gl);

            text(Canvas::WHITE, 30, &"Blair Witch", &mut cache, context.transform.trans(100.0, 90.0), gl);
        });
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.background.animate();
    }

    pub fn input(&mut self, b: Button) {
        match b {
            Button::Keyboard(Key::Up) => {
                self.move_up()
            }
            Button::Keyboard(Key::Down) => {
                self.move_down()
            }
            Button::Keyboard(Key::Left) => {
                self.move_left()
            }
            Button::Keyboard(Key::Right) => {
                self.move_right()
            }
            _ => ()
        }
    }

    fn move_left(&mut self) {
        if self.horizontal > 0.0 {
            self.horizontal -= Canvas::W_SPEED;
        }
    }

    fn move_right(&mut self) {
        let max: f64 = self.background.levels[0].get_width() as f64;
        if self.horizontal < max {
            self.horizontal += Canvas::W_SPEED;
        }
    }

    fn move_up(&mut self) {
        if self.vertical > 0.0 {
            self.vertical -= Canvas::W_SPEED;
        }
    }

    fn move_down(&mut self) {
        let max: f64 = self.background.levels[0].get_height() as f64;
        if self.vertical < max {
            self.vertical += Canvas::W_SPEED
        }
    }
}