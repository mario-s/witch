extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::*;
use game::assets::*;

pub struct Canvas {
    gl: GlGraphics
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

        self.gl.draw(viewport, |context, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);


            rectangle(RED, [10.0, 10.0, 50.0, 80.0],
                      context.transform,
                      gl);

            text(BLACK, 30, &"Test", &mut cache, context.transform.trans(150.0, 80.0), gl);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        // update code here

    }

}