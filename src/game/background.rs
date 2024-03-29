extern crate opengl_graphics;

use graphics::ImageSize;
use opengl_graphics::Texture;
use game::assets::Assets;

pub struct Background {
    pub levels: [Texture; 10],
    pub x_shifts: [f64; 10],
}

impl Background {
    pub fn new() -> Background {
        Background {
            levels: [
                Assets::texture("10_Sky.png"),
                Assets::texture("09_Forest.png"),
                Assets::texture("08_Forest.png"),
                Assets::texture("07_Forest.png"),
                Assets::texture("06_Forest.png"),
                Assets::texture("05_Particles.png"),
                Assets::texture("04_Forest.png"),
                Assets::texture("03_Particles.png"),
                Assets::texture("02_Bushes.png"),
                Assets::texture("01_Mist.png"),
            ],
            x_shifts: [0.0; 10],
        }
    }

    pub fn get_dimension(&self) -> [f64; 2] {
        [self.get_width(), self.get_height()]
    }

    fn get_width(&self) -> f64 {
        self.levels[0].get_width() as f64
    }

    fn get_height(&self) -> f64 {
        self.levels[0].get_height() as f64
    }

    ///This method will update the x value of the image location.
    pub fn update(&mut self) {
        let speed = 0.05;
        for i in 0..self.levels.len() {
            let f = (1 + i) as f64;
            self.x_shifts[i] -= speed * f;
        }
        self.reset();
    }

    fn reset(&mut self) {
        let min: f64 = -1.0 * self.get_width();
        for i in 0..self.levels.len() {
            if self.x_shifts[i] < min {
                self.x_shifts[i] = 0.0;
            }
        }
    }
}
