extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use std::error::Error;
use std::str::FromStr;
use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::window::BuildFromWindowSettings;

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

fn window() -> OpenGlWindow {
    let versions: [&str; 2] = ["3.1", "3.2"];
    for v in versions.iter() {
        let opengl = OpenGL::from_str(v).unwrap();
        let result: Result<Window, Box<dyn Error>> = build(opengl);
        //if the result is ok, a supported opengl version is available
        if result.is_ok() {
            return OpenGlWindow {
                opengl: opengl,
                window: result.unwrap(), 
            }
        }
    }
    panic!("No supported OpenGl version found! {:?}", versions);    
}

//try to build a window
fn build<W: BuildFromWindowSettings>(opengl: OpenGL) -> Result<W, Box<dyn Error>> {
    WindowSettings::new("super.mario",[272, 160])
        .resizable(false).exit_on_esc(true).graphics_api(opengl)
        .build()
}
