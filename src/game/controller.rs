use piston::input::*;

const VELOCITY: f64 = 10.0;

#[derive(Debug, PartialEq, Eq)]
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

/// This controls the movements of the character
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
    dt: f64,
    at_horizontal_edge: bool,
    at_vertical_edge: bool
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
            dt: 0.0,
            at_horizontal_edge: false,
            at_vertical_edge: false
        }
    }

    pub fn key_event(&mut self, s: ButtonState, k: Key) {
        //state changed
        if self.state != s {
            //println!("new state: {:?}", s);
            self.state = s;
            //a press event
            if self.state == ButtonState::Press {
                self.change_direction(k)
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
        match self.state {
            ButtonState::Press => {
                if self.dt < 1.0 {
                    self.dt = self.dt + dt;
                }
            },
            ButtonState::Release => {
                if self.dt > 0.0 {
                    self.slow_down(dt);
                }
            }
        }
        self.update_position();
    }

    //decelarate the player
    fn slow_down(&mut self, dt: f64) {
        //decelerate when player is not at the edge
        if !self.at_vertical_edge && !self.at_horizontal_edge {
            let v = self.dt - (dt * 2.0);
            //when it is smaller than a minimum we will set it to 0.0
            if v < 0.01 {
                self.dt = 0.0;
            } else {
                self.dt = v;
            }
        } else {
            //full stop
            self.dt = 0.0;
        }
    }

    fn update_position(&mut self) {
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

    fn move_horizontal(&mut self, velo: f64) {
        let next: f64 = self.horizontal + (self.dt * velo);
        //println!("horizontal: {:?}", next);
        if self.in_frame(next, self.min_horizontal, self.max_horizontal) {
            self.horizontal = next;
            self.at_horizontal_edge = false;
        } else {
            self.at_horizontal_edge = true;
        }
    }

    fn move_vertical(&mut self, velo: f64) {
        let next: f64 = self.vertical + (self.dt * velo);
        //println!("vertical: {:?}", next);
        if self.in_frame(next, self.min_vertical, self.max_vertical) {
            self.vertical = next;
            self.at_vertical_edge = false;
        } else {
            self.at_vertical_edge = true;
        }
    }

    fn in_frame(&self, next: f64, min: f64, max: f64) -> bool {
        return next >= min && next <= max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Controller {
        Controller::new(2, 2, 0.0, 0.0, 20.0, 20.0)
    }

    #[test]
    fn controller_move_up() {
        let mut c = setup();
        c.key_event(ButtonState::Press, Key::Up);
        assert_eq!(c.direction, Direction::N);
    }

    #[test]
    fn controller_move_down() {
        let mut c = setup();
        c.key_event(ButtonState::Press, Key::Down);
        assert_eq!(c.direction, Direction::S);
    }

    #[test]
    fn controller_move_right() {
        let mut c = setup();
        c.key_event(ButtonState::Press, Key::Right);
        assert_eq!(c.direction, Direction::E);
    }

    #[test]
    fn controller_move_left() {
        let mut c = setup();
        c.key_event(ButtonState::Press, Key::Left);
        assert_eq!(c.direction, Direction::W);
    }

    #[test]
    fn controller_time_event() {
        let mut c = setup();
        c.key_event(ButtonState::Press, Key::Right);
        c.time_event(0.1);
        assert!(c.horizontal != 0.0);
    }
}