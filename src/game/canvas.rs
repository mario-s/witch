extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use sprite::*;
use std::rc::Rc;
use piston::window::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::*;


use game::assets::Assets;

pub struct Canvas {
    gl: GlGraphics,
}

impl Canvas {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    pub fn new(opengl: OpenGL) -> Canvas {
        Canvas{gl: GlGraphics::new(opengl)}
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, args: RenderArgs) {
        let viewport = args.viewport();
        let mut cache = GlyphCache::new(Assets::assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();

        let back_texture = Assets::texture("parallax-forest-back-trees.png");
        let middle_texture = Assets::texture("parallax-forest-middle-trees.png");
        let front_texture = Assets::texture("parallax-forest-front-trees.png");
        let light_texture = Assets::texture("parallax-forest-lights.png");


        let witch = Rc::new(Assets::icon("witch-icon.png"));
        let mut sprite = Sprite::from_texture(witch.clone());
        sprite.set_position(50.0, 80.0);

        let mut scene = Scene::new();
        scene.add_child(sprite);

        self.gl.draw(viewport, |context, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);

            image(&back_texture, context.transform, gl);
            image(&middle_texture, context.transform, gl);
            image(&light_texture, context.transform, gl);
            image(&front_texture, context.transform, gl);

            scene.draw(context.transform, gl);

            text(Canvas::BLACK, 30, &"Blair Witch", &mut cache, context.transform.trans(100.0, 90.0), gl);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // update code here

    }

}