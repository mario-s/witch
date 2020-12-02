use piston::input::*;

const WITCH_SPEED: f64 = 15.0;

#[derive(Debug)]
pub struct Controller {
    pub horizontal: f64,
    pub vertical: f64,
    min_horizontal: f64,
    max_horizontal: f64,
    min_vertical: f64,
    max_vertical: f64,
    moving: bool
}

impl Controller {
    pub fn new(fig_width: u32, fig_height: u32,
        horizontal: f64, vertical: f64,
        width: f64, height: f64) -> Controller {

        Controller {
            horizontal,
            vertical,
            min_horizontal: fig_width as f64 / 2.0,
            max_horizontal: width - fig_width as f64 / 2.0,
            min_vertical: fig_height as f64 / 2.0,
            max_vertical: height - fig_height as f64 / 2.0,
            moving: false
        }
    }

    pub fn do_move(&mut self,  k: Key, state: ButtonState) {
        self.moving = !self.moving;
        println!("Pressed keyboard key '{:?}'", self.moving);
        match k {
            Key::Up => {
                self.move_vertical(-WITCH_SPEED)
            }
            Key::Down => {
                self.move_vertical(WITCH_SPEED)
            }
            Key::Left => {
                self.move_horizontal(-WITCH_SPEED)
            }
            Key::Right => {
                self.move_horizontal(WITCH_SPEED)
            }
            Key::E => {
                //right up
                self.move_horizontal(WITCH_SPEED);
                self.move_vertical(-WITCH_SPEED);
            }
            Key::Q => {
                //left up
                self.move_horizontal(-WITCH_SPEED);
                self.move_vertical(-WITCH_SPEED);
            }
            Key::D => {
                //right down
                self.move_horizontal(WITCH_SPEED);
                self.move_vertical(WITCH_SPEED);
            }
            Key::A => {
                //left down
                self.move_horizontal(-WITCH_SPEED);
                self.move_vertical(WITCH_SPEED);
            }
            _ => ()
        }
    }

    fn move_horizontal(&mut self, d_x: f64) {
        let next: f64 = self.horizontal + d_x;
        if next >= self.min_horizontal && next <= self.max_horizontal {
            self.horizontal = next;
        }
    }

    fn move_vertical(&mut self, d_y: f64) {
        let next: f64 = self.vertical + d_y;
        if next >= self.min_vertical && next <= self.max_vertical {
            self.vertical = next;
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Controller {
        Controller::new(2, 2, 0.0, 0.0, 20.0, 20.0)
    } 

    #[test]
    fn controller_move_down() {
        let mut c = setup();
        c.do_move(Key::Down);
        assert_eq!(c.vertical, WITCH_SPEED);
    }

    #[test]
    fn controller_move_right() {
        let mut c = setup();
        c.do_move(Key::Right);
        assert_eq!(c.horizontal, WITCH_SPEED);
    }
}