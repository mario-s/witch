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

//window with the version of OpenGL
struct OpenGlWindow {
    opengl: OpenGL,
    window: Window,
}

fn main() {
    let win: OpenGlWindow = window();
    let mut window: Window = win.window;

    let mut canvas = game::Canvas::new(win.opengl);
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

//try to build a window
fn window() -> OpenGlWindow {
    //initial version of OpenGL
    let mut opengl = OpenGL::V3_1;
    let result = build(opengl);
    //if the result is success return it, if not try with another version of OpenGL. 
    //Give up if that also fails.
    let window = match result {
        Ok(win) => win,
        Err(_err) => {
            opengl = OpenGL::V3_2;
            build(opengl).unwrap()
        }
    };

    return OpenGlWindow {
        opengl: opengl,
        window: window, 
    }
}

fn build<W: BuildFromWindowSettings>(opengl: OpenGL) -> Result<W, String> {
    return WindowSettings::new("super.mario",[272, 160])
        .opengl(opengl).resizable(false).exit_on_esc(true)
        .build();
}
