extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;
extern crate music;

use std::error::Error;
use std::str::FromStr;
use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::window::BuildFromWindowSettings;

mod game;
use game::Canvas;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Synth,
}

//window with the version of OpenGL
struct OpenGlWindow {
    opengl: OpenGL,
    window: Window,
}

fn main() {
    music::start::<Music, Music ,_>(32, || {
        music::bind_music_file(Music::Synth, "./assets/sound/rise.wav");
        music::set_volume(music::MAX_VOLUME);

        let win: OpenGlWindow = window();
        let mut window: Window = win.window;
        let mut canvas = Canvas::new(win.opengl);
        let mut events = Events::new(EventSettings::new());

        music::play_music(&Music::Synth, music::Repeat::Forever);

        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                canvas.render(r);
            }
    
            if let Some(u) = e.update_args() {
                canvas.update(u);
            }
    
            if let Some(b) = e.press_args() {
                canvas.toggle(b);
                match canvas.pause {
                    false => music::set_volume(music::MIN_VOLUME),
                    _     => music::set_volume(music::MAX_VOLUME),
                };
            }

            if let Some(k) = e.button_args() {
                if let Button::Keyboard(key) = k.button {
                    canvas.input(k.state, key);
                }
            }
        }
    });
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
    WindowSettings::new("super.mario",[1280, 720])
        .resizable(false).exit_on_esc(true).graphics_api(opengl)
        .build()
}
