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


pub struct Canvas {
    gl: GlGraphics,
    translation: f64,
    backgrounds: [Texture; 4]
}


impl Canvas {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    pub fn new(opengl: OpenGL) -> Canvas {
        Canvas{
            gl: GlGraphics::new(opengl),
            translation: 0.0,
            backgrounds:  [
                Assets::texture("parallax-forest-back-trees.png"),
                Assets::texture("parallax-forest-middle-trees.png"),
                Assets::texture("parallax-forest-lights.png"),
                Assets::texture("parallax-forest-front-trees.png"),
            ]
        }
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, args: RenderArgs) {
        let viewport = args.viewport();
        let mut cache = GlyphCache::new(Assets::assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();

        let mut scene = Scene::new();
        scene.add_child(Canvas::witch());

        let translation = self.translation;
        let imgs = &self.backgrounds;

        self.gl.draw(viewport, |context, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);

            for texture in imgs.into_iter() {
                image(texture, context.transform, gl);
            }

            let trans = context.transform.trans(translation, 0.0);
            scene.draw(trans, gl);

            text(Canvas::BLACK, 30, &"Blair Witch", &mut cache, context.transform.trans(100.0, 90.0), gl);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // update code here
        self.translation += 0.1;
    }


    fn witch() -> Sprite<Texture> {
        let witch = Rc::new(Assets::icon("witch-icon.png"));
        let mut sprite = Sprite::from_texture(witch.clone());
        sprite.set_position(50.0, 80.0);
        return sprite;
    }
}