use piston::input::*;

const VELOCITY: f64 = 15.0;

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    None
}

#[derive(Debug)]
pub struct Controller {
    pub horizontal: f64,
    pub vertical: f64,
    min_horizontal: f64,
    max_horizontal: f64,
    min_vertical: f64,
    max_vertical: f64,
    state: ButtonState,
    direction: Direction,
    dt: f64
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
            state: ButtonState::Release,
            direction: Direction::None,
            dt: 0.0
        }
    }

    pub fn key_event(&mut self, s: ButtonState, k: Key) {
        //state changed
        if self.state != s {
            //println!("new state: {:?}", s);
            self.state = s;
            //a press event
            if self.state == ButtonState::Press {
                self.change_direction(k);
            }
        }
    }

    fn change_direction(&mut self, k: Key) {
        match k {
            Key::Up => self.direction = Direction::N,
            Key::Down => self.direction = Direction::S,
            Key::Left => self.direction = Direction::W,
            Key::Right => self.direction = Direction::E,
            Key::E => self.direction = Direction::NE,
            Key::Q => self.direction = Direction::NW,
            Key::D => self.direction = Direction::SE,
            Key::A => self.direction = Direction::SW,
            _ => self.direction = Direction::None
        }
    }

    pub fn time_event(&mut self, dt: f64) {
        if self.state == ButtonState::Press {
            self.update_position(dt * 10.0);
        }
    }

    fn update_position(&mut self, dt: f64) {
        self.dt = dt;
        match self.direction {
            Direction::N => self.move_vertical(-VELOCITY),
            Direction::S => self.move_vertical(VELOCITY),
            Direction::W => self.move_horizontal(-VELOCITY),
            Direction::E => self.move_horizontal(VELOCITY),
            Direction::NE => {
                self.move_horizontal(VELOCITY);
                self.move_vertical(-VELOCITY);
            },
            Direction::NW => {
                self.move_horizontal(-VELOCITY);
                self.move_vertical(-VELOCITY);
            },
            Direction::SE => {
                self.move_horizontal(VELOCITY);
                self.move_vertical(VELOCITY);
            },
            Direction::SW => {
                self.move_horizontal(-VELOCITY);
                self.move_vertical(VELOCITY);
            }
            _ => {}
        }
    }


    fn move_horizontal(&mut self, dx: f64) {
        let next: f64 = self.horizontal + (self.dt * dx);
        //println!("horizontal: {:?}", next);
        if next >= self.min_horizontal && next <= self.max_horizontal {
            self.horizontal = next;
        }
    }

    fn move_vertical(&mut self, dy: f64) {
        let next: f64 = self.vertical + (self.dt * dy);
        //println!("vertical: {:?}", next);
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
        c.key_event(ButtonState::Press, Key::Down);
        assert_eq!(c.vertical, VELOCITY);
    }

    #[test]
    fn controller_move_right() {
        let mut c = setup();
        c.key_event(ButtonState::Press, Key::Right);
        assert_eq!(c.horizontal, VELOCITY);
    }
}