pub mod ghost;
pub mod map;

use std::convert::TryFrom;

use crate::sim::Simulator;

use self::map::Map;
use self::map::Tile;

use self::ghost::{Ghost, GhostMode, Ghosts, Interaction};

// const START_POS: (i32, i32) = ((map::MAP_WIDTH - 2) as i32, (map::MAP_HEIGHT - 2) as i32);

// Parameters
const SCORE_PELLET: u32 = 10;
const SCORE_PU: u32 = 50;
const SCORE_GHOST: u32 = 200;

/// Constants that do not change while the game is running.
#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    pub start_pos: (i32, i32),
    pub start_dir: Direction,
}

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

pub struct Game<RG: Simulator> {
    params: Parameters,
    map: Map,
    lives: u8,
    score: u32,
    level: usize,
    x: i32,
    y: i32,
    direction: Direction,
    direction_intent: Direction,
    ghosts: Ghosts,
    ticks: u32,
    paused: bool,
    rg: RG,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vector(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

pub struct Stats {
    pub lives: u8,
    pub score: u32,
    pub level: usize,
}

impl<RG: Simulator> Game<RG> {
    pub fn new(params: Parameters, mut sim: RG) -> Self {
        // let mut sim = RG::default();
        let map_file = sim.load_file("map.txt");

        Game {
            params,
            map: Map::new(map_file),
            lives: 5,
            score: 0,
            level: 1,
            x: params.start_pos.0,
            y: params.start_pos.1,
            direction: params.start_dir,
            direction_intent: params.start_dir,
            ghosts: Ghosts::new(),
            ticks: 0,
            paused: false,
            rg: sim,
        }
    }

    pub fn input(&mut self, input: Input) -> bool {
        let mut should_quit = false;
        match input {
            Input::Up => self.set_direction_intent(Direction::Up),
            Input::Down => self.set_direction_intent(Direction::Down),
            Input::Left => self.set_direction_intent(Direction::Left),
            Input::Right => self.set_direction_intent(Direction::Right),
            Input::Quit => should_quit = true,
            Input::Pause => self.paused = !self.paused,
            Input::None => (),
        }
        return should_quit;
    }

    pub fn update(&mut self) {
        if !self.paused {
            println!("score: {}", self.stats().score);
            self.tick();
        }
    }

    pub fn get_player(&self) -> (i32, i32, Direction) {
        self.player()
    }

    pub fn get_map(&self) -> &Map {
        self.map()
    }

    pub fn get_ghosts(&self) -> &[Ghost] {
        self.ghosts()
    }

    pub fn frightened(&self) -> bool {
        self.ghost_mode() == GhostMode::Frightened
    }

    pub fn get_stats(&self) -> Stats {
        self.stats()
    }

    pub fn set_direction_intent(&mut self, direction: Direction) {
        if self.lives == 0 {
            return;
        }
        self.direction_intent = direction;
        if self.can_turn() {
            self.direction = self.direction_intent;
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        if self.map.pellets() == 0 {
            self.advance_level();
            return;
        }
        if self.lives == 0 {
            return;
        }
        self.move_pacman();
        self.ghosts
            .move_ghosts(&self.map, (self.x, self.y, self.direction), &mut self.rg);

        match self.ghosts.interact_with_player((self.x, self.y)) {
            Some(Interaction::KillPlayer) => {
                self.x = self.params.start_pos.0;
                self.y = self.params.start_pos.1;
                // Do we also want to set start direction here?
                self.lives -= 1;
            }
            Some(Interaction::KillGhost(n)) => {
                self.score += SCORE_GHOST * n as u32;
            }
            None => (),
        }
    }

    fn move_pacman(&mut self) {
        if self.can_turn() {
            self.direction = self.direction_intent;
        }
        let (x, y) = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        if !self.map.is_wall(x, y) {
            self.x = x;
            self.y = y;
        }
        match self.map.get(x, y) {
            None => {
                if x == -1 {
                    self.x = self.map.width as i32 - 1;
                } else if x == self.map.width as i32 {
                    self.x = 0;
                }
            }
            Some(Tile::Empty) => (),
            Some(Tile::Dot) => {
                self.map.consume(x, y);
                self.score += SCORE_PELLET;
            }
            Some(Tile::PowerUp) => {
                self.map.consume(x, y);
                self.ghosts.frighten();
                self.score += SCORE_PU;
            }
            _ => (),
        }
    }

    fn can_turn(&self) -> bool {
        let (x, y) = match self.direction_intent {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        match self.map.get(x, y) {
            None => false,
            Some(Tile::Wall) => false,
            _ => true,
        }
    }

    fn advance_level(&mut self) {
        self.level += 1;
        self.x = self.params.start_pos.0;
        self.y = self.params.start_pos.1;
        self.ghosts.reset();
        self.map.reset();
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn player(&self) -> (i32, i32, Direction) {
        (self.x, self.y, self.direction_intent)
    }

    pub fn ghosts(&self) -> &[Ghost] {
        &self.ghosts.get()
    }

    pub fn ghost_mode(&self) -> GhostMode {
        self.ghosts.ghost_mode()
    }

    pub fn stats(&self) -> Stats {
        Stats {
            lives: self.lives,
            score: self.score,
            level: self.level,
        }
    }
}

// // DEBUG VIEWS
// #[allow(dead_code)]
// impl Game {
//     pub fn ghost_targets(&self) -> [(i32, i32); 4] {
//         self.ghosts.targets((self.x, self.y, self.direction))
//     }

//     pub fn level_up(&mut self) {
//         self.map.remove_all_pellets();
//     }
// }
