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

use game::assets::*;

pub struct Canvas {
    gl: GlGraphics,
    background: Background,
    horizontal: f64,
    vertical: f64,
    witch: Rc<Texture>,
    pause: bool
}

impl Canvas {
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const W_SPEED: f64 = 4.0;
    //start position of the sprite
    const W_X: f64 = 50.0;
    const W_Y: f64 = 80.0;
    //max allowed left position
    const M_LEFT: f64 = -10.0;

    pub fn new(opengl: OpenGL) -> Canvas {
        Canvas {
            gl: GlGraphics::new(opengl),
            background: Background::new(),
            horizontal: 0.0,
            vertical: 0.0,
            witch: Rc::new(Assets::icon("witch-icon.png")),
            pause: true,
        }
    }

    fn witch(&mut self) -> Sprite<Texture> {
        let mut sprite = Sprite::from_texture(self.witch.clone());
        sprite.set_position(Canvas::W_X, Canvas::W_Y);
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
        let pause = self.pause;
        let mut index = 0;

        self.gl.draw(viewport, |c, g| {
            clear(Canvas::WHITE, g);

            for texture in imgs.into_iter() {
                let translation = translations[index];
                let width: f64 = texture.get_width() as f64;
                image(texture, c.transform.trans(translation, 0.0), g);
                image(texture, c.transform.trans(width + translation, 0.0), g);
                index += 1;
            }

            let trans = c.transform.trans(horizontal, vertical);
            scene.draw(trans, g);

            if pause {
                text(Canvas::WHITE, 30, &"Blair Witch", &mut cache, c.transform.trans(100.0, 90.0), g);
            }
        });
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.background.animate();
    }

    pub fn input(&mut self, b: Button) {
        println!("Pressed keyboard key '{:?}'", b);
        if !self.pause {
            self.do_move(b);
        }
        
        self.toggle(b);
    }

    fn toggle(&mut self,  b: Button) {
        if b == Button::Keyboard(Key::P) {
            self.pause = !self.pause;
        }
    }

    fn do_move(&mut self, b: Button) {
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
        if self.horizontal > Canvas::M_LEFT {
            self.horizontal -= Canvas::W_SPEED;
        }
    }

    fn move_right(&mut self) {
        let max: f64 = self.background.get_width() - Canvas::W_Y;
        if self.horizontal < max {
            self.horizontal += Canvas::W_SPEED;
        }
    }

    fn move_up(&mut self) {
        if self.vertical > -Canvas::W_X {
            self.vertical -= Canvas::W_SPEED;
        }
    }

    fn move_down(&mut self) {
        if self.vertical < Canvas::W_X {
            self.vertical += Canvas::W_SPEED
        }
    }
}