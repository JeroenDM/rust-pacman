use std::convert::TryFrom;

use crate::pacman::{ghost::Ghost, ghost::GhostMode, map::Map, Direction, Pacman, Stats};
use piston::input::Button;
use piston::input::Event;
use piston::input::{PressEvent, UpdateEvent};
use piston::UpdateArgs;

#[derive(Debug, Clone, Copy)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Quit,
    Pause,
    None,
}

impl From<Input> for char {
    fn from(val: Input) -> Self {
        match val {
            Input::Up => 'u',
            Input::Down => 'd',
            Input::Left => 'l',
            Input::Right => 'r',
            Input::Quit => 'q',
            Input::Pause => 'p',
            Input::None => 'n',
        }
    }
}

impl TryFrom<char> for Input {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'u' => Ok(Input::Up),
            'd' => Ok(Input::Down),
            'l' => Ok(Input::Left),
            'r' => Ok(Input::Right),
            'q' => Ok(Input::Quit),
            'p' => Ok(Input::Pause),
            _ => Err(format!("Invalid input character: '{}'", c)),
        }
    }
}

pub struct Controller {
    game: Pacman,
    delta: f64,
    paused: bool,
}

impl Controller {
    pub fn new(game: Pacman) -> Self {
        Controller {
            game,
            delta: 0.0,
            paused: false,
        }
    }

    pub fn input(&mut self, input: Input) -> bool {
        let mut should_quit = false;
        match input {
            Input::Up => self.game.set_direction_intent(Direction::Up),
            Input::Down => self.game.set_direction_intent(Direction::Down),
            Input::Left => self.game.set_direction_intent(Direction::Left),
            Input::Right => self.game.set_direction_intent(Direction::Right),
            Input::Quit => should_quit = true,
            Input::Pause => self.paused = !self.paused,
            Input::None => (),
        }
        return should_quit;
    }

    pub fn update(&mut self, u: UpdateArgs) {
        self.delta += u.dt;
        if self.delta > 0.25 {
            self.delta -= 0.25;
            if !self.paused {
                println!("tick!");
                self.game.tick();
            }
        }
    }

    pub fn get_player(&self) -> (i32, i32, Direction) {
        self.game.player()
    }

    pub fn get_map(&self) -> &Map {
        self.game.map()
    }

    pub fn get_ghosts(&self) -> &[Ghost] {
        self.game.ghosts()
    }

    pub fn frightened(&self) -> bool {
        self.game.ghost_mode() == GhostMode::Frightened
    }

    pub fn get_stats(&self) -> Stats {
        self.game.stats()
    }
}

// DEBUG VIEWS
#[allow(dead_code)]
impl Controller {
    pub fn ghost_targets(&self) -> [(i32, i32); 4] {
        self.game.ghost_targets()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}
