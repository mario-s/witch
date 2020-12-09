use piston::input::*;

const VELOCITY: f64 = 10.0;

#[derive(Debug, PartialEq)]
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
            match s {
                ButtonState::Press => self.change_direction(k),
                ButtonState::Release => self.dt = 0.0
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
            self.update_position(dt);
        }
    }

    fn update_position(&mut self, dt: f64) {
        self.dt = self.dt + dt;
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
        }
    }

    fn move_vertical(&mut self, velo: f64) {
        let next: f64 = self.vertical + (self.dt * velo);
        //println!("vertical: {:?}", next);
        if self.in_frame(next, self.min_vertical, self.max_vertical) {
            self.vertical = next;
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
        c.time_event(0.01);
        assert!(c.horizontal != 0.0);
    }
}