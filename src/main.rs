extern crate piston_window;
extern crate find_folder;

use std::path::PathBuf;
use piston_window::*;
use piston_window::text::Text;


fn new_window() -> PistonWindow {
    let opengl = OpenGL::V3_2;
    return WindowSettings::new("Hello Piston!", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
}

fn assets(path: &str) -> PathBuf {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    return assets.join(path);
}

fn main() {
    let black = [0.0, 0.0, 0.0, 1.0];
    let red = [1.0, 0.0, 0.0, 0.8];

    let mut window: PistonWindow = new_window();

    let settings = TextureSettings::new();
    let mut glyphs = Glyphs::new(assets("FreeSans.ttf"), window.factory.clone(), settings).unwrap();


    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            rectangle(red, [10.0, 10.0, 50.0, 80.0],
                      context.transform,
                      graphics);

            Text::new_color(black, 70)
                .draw(
                    &"P",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(15.0, 80.0),
                    graphics);

            Text::new_color(black, 50)
                .draw(
                    &"iston Tutorial",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(60.0, 80.0),
                    graphics);

            Text::new_color(black, 15)
                .draw(
                    &"<ESC> to exit",
                    &mut glyphs,
                    &context.draw_state,
                    context.transform.trans(540.0, 470.0),
                    graphics);
        });
    }
}
