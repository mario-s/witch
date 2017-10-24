extern crate find_folder;
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;


use std::path::PathBuf;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::*;



struct Canvas {
    gl: GlGraphics
}

impl Canvas {
    fn new(opengl: OpenGL) -> Canvas {
        Canvas{gl: GlGraphics::new(opengl)}
    }

    fn render(&mut self, args: RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let viewport = args.viewport();
        let mut cache = GlyphCache::new(assets("FreeSans.ttf"), (), TextureSettings::new()).unwrap();

        self.gl.draw(viewport, |context, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);
            // render code here
            rectangle(RED, [10.0, 10.0, 50.0, 80.0],
                      context.transform,
                      gl);

            text(BLACK, 20, "Test", &mut cache, context.transform.trans(150.0, 80.0), gl);
        });
    }

    fn update(&mut self, args: UpdateArgs) {
        // update code here

    }

}

fn assets(path: &str) -> PathBuf {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    return assets.join(path);
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Demo",
        [640, 480]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut canvas = Canvas::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            canvas.render(r)
        }

        if let Some(u) = e.update_args() {
            canvas.update(u);
        }
    }
}
