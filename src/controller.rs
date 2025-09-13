use crate::pacman::{ghost::Ghost, ghost::GhostMode, map::Map, Direction, Pacman, Stats};
use piston::input::Button;
use piston::input::Event;
use piston::input::{PressEvent, UpdateEvent};
use piston::UpdateArgs;

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

    pub fn input(&mut self, button: Button) -> bool {
        use piston::input::keyboard::Key;
        let mut should_quit = false;
        match button {
            Button::Keyboard(Key::Up) | Button::Keyboard(Key::I) => {
                self.game.set_direction_intent(Direction::Up)
            }
            Button::Keyboard(Key::Down) | Button::Keyboard(Key::K) => {
                self.game.set_direction_intent(Direction::Down)
            }
            Button::Keyboard(Key::Left) | Button::Keyboard(Key::J) => {
                self.game.set_direction_intent(Direction::Left)
            }
            Button::Keyboard(Key::Right) | Button::Keyboard(Key::L) => {
                self.game.set_direction_intent(Direction::Right)
            }
            Button::Keyboard(Key::Q) => should_quit = true,
            Button::Keyboard(Key::P) => self.paused = !self.paused,
            // Button::Keyboard(Key::U) => self.game.level_up(),
            _ => (),
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
