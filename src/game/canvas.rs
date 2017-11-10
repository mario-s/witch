extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::*;

use game::assets::Assets;

pub struct Canvas {
    gl: GlGraphics,
}

impl Canvas {
    pub fn new(opengl: OpenGL) -> Canvas {
        Canvas{gl: GlGraphics::new(opengl)}
    }

    #[allow(unused_must_use)]
    pub fn render(&mut self, args: RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let viewport = args.viewport();
        let mut cache = GlyphCache::new(Assets::assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();
        let back_texture = Assets::texture("parallax-forest-back-trees.png");
        let middle_texture = Assets::texture("parallax-forest-middle-trees.png");
        let front_texture = Assets::texture("parallax-forest-front-trees.png");
        let light_texture = Assets::texture("parallax-forest-lights.png");

        self.gl.draw(viewport, |context, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);

            image(&back_texture, context.transform, gl);
            image(&middle_texture, context.transform, gl);
            image(&light_texture, context.transform, gl);
            image(&front_texture, context.transform, gl);


            rectangle(RED, [20.0, 20.0, 50.0, 80.0],
                      context.transform,
                      gl);

            text(BLACK, 30, &"Blair Witch", &mut cache, context.transform.trans(100.0, 90.0), gl);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // update code here

    }

}