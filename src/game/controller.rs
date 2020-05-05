use piston::input::*;

use game::sprites::*;

const WITCH_SPEED: f64 = 5.0;
//max allowed left position
const MAX_LEFT: f64 = -10.0;

#[derive(Debug)]
pub struct Controller {
    pub horizontal: f64,
    pub vertical: f64,
    max_right: f64,
    max_bottom: f64
}

impl Controller {
    pub fn new(width: f64, height: f64) -> Controller {

        Controller {
            horizontal: 0.0,
            vertical: 0.0,
            max_right: width - WITCH_Y,
            max_bottom: height
        }
    }

    pub fn do_move(&mut self, b: Button) {
        match b {
            Button::Keyboard(Key::Up) => {
                //up
                self.move_vertical(-WITCH_SPEED)
            }
            Button::Keyboard(Key::Down) => {
                //down
                self.move_vertical(WITCH_SPEED)
            }
            Button::Keyboard(Key::Left) => {
                //left
                self.move_horizontal(-WITCH_SPEED)
            }
            Button::Keyboard(Key::Right) => {
                //right
                self.move_horizontal(WITCH_SPEED)
            }
            Button::Keyboard(Key::E) => {
                //right up
                self.move_horizontal(WITCH_SPEED);
                self.move_vertical(-WITCH_SPEED);
            }
            Button::Keyboard(Key::Q) => {
                //left up
                self.move_horizontal(-WITCH_SPEED);
                self.move_vertical(-WITCH_SPEED);
            }
            Button::Keyboard(Key::D) => {
                //right down
                self.move_horizontal(WITCH_SPEED);
                self.move_vertical(WITCH_SPEED);
            }
            Button::Keyboard(Key::A) => {
                //left down
                self.move_horizontal(-WITCH_SPEED);
                self.move_vertical(WITCH_SPEED);
            }
            _ => ()
        }
    }

    fn move_horizontal(&mut self, d_x: f64) {
        let next: f64 = self.horizontal + d_x;
        if next >= MAX_LEFT && next <= self.max_right {
            self.horizontal = next;
        }
    }

    fn move_vertical(&mut self, d_y: f64) {
        let next: f64 = self.vertical + d_y;
        if next >= -WITCH_X && next <= self.max_bottom {
            self.vertical = next;
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Controller {
        return Controller::new(WITCH_Y + 5.0, WITCH_X);
    } 

    #[test]
    fn controller_move_down() {
        let mut c = setup();
        c.do_move(Button::Keyboard(Key::Down));
        assert_eq!(c.vertical, WITCH_SPEED);
    }

    #[test]
    fn controller_move_right() {
        let mut c = setup();
        c.do_move(Button::Keyboard(Key::Right));
        assert_eq!(c.horizontal, WITCH_SPEED);
    }
}