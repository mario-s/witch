extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;


use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

mod game;

fn main() {
    let mut opengl = OpenGL::V3_1;
    let result = build(opengl);
    let mut window: Window = match result {
        Ok(win) => win,
        Err(_err) => {
            opengl = OpenGL::V3_2;
            build(opengl).unwrap()
        }
    };

    let mut canvas = game::Canvas::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            canvas.render(r);
        }

        if let Some(u) = e.update_args() {
            canvas.update(u);
        }

        if let Some(p) = e.press_args() {
            canvas.input(p);
        }
    }
}

fn build<W: BuildFromWindowSettings>(opengl: OpenGL) -> Result<W, String> {
    return WindowSettings::new("super.mario",[272, 160])
        .opengl(opengl).resizable(false).exit_on_esc(true)
        .build();
}
