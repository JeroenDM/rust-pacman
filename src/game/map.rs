// const MAP_STR: [&'static str; 31] = [
//     "############################",
//     "#............##............#",
//     "#.####.#####.##.#####.####.#",
//     "#X####.#####.##.#####.####X#",
//     "#.####.#####.##.#####.####.#",
//     "#..........................#",
//     "#.####.##.########.##.####.#",
//     "#.####.##.########.##.####.#",
//     "#......##....##....##......#",
//     "######.##### ## #####.######",
//     "######.##### ## #####.######",
//     "######.##          ##.######",
//     "######.## ###HH### ##.######",
//     "######.## #HHHHHH# ##.######",
//     "      .   #HHHHHH#   .      ",
//     "######.## #HHHHHH# ##.######",
//     "######.## ######## ##.######",
//     "######.##          ##.######",
//     "######.## ######## ##.######",
//     "######.## ######## ##.######",
//     "#............##............#",
//     "#.####.#####.##.#####.####.#",
//     "#.####.#####.##.#####.####.#",
//     "#X..##................##..X#",
//     "###.##.##.########.##.##.###",
//     "###.##.##.########.##.##.###",
//     "#......##....##....##......#",
//     "#.##########.##.##########.#",
//     "#.##########.##.##########.#",
//     "#..........................#",
//     "############################",
// ];
const MAP_STR: [&'static str; 10] = [
    "############################",
    "#................X.........#",
    "#..........................#",
    "#..........................#",
    "#..........................#",
    "#..............X.X.........#",
    "#..........................#",
    "#..........................#",
    "#..........................#",
    "############################",
];

// TODO remove usage of these constants from gamp.rs.
pub const MAP_WIDTH: usize = MAP_STR[0].len();
pub const MAP_HEIGHT: usize = MAP_STR.len();

fn pellet_coords() -> Vec<(usize, usize)> {
    MAP_STR
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '.')
                .map(move |(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .fold(vec![], |mut acc, mut l| {
            acc.append(&mut l);
            acc
        })
}

pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    pellets: u32,
    pellet_coords: Vec<(usize, usize)>,
}

#[derive(Clone, Copy)]
pub enum Tile {
    Wall,
    House,
    Dot,
    PowerUp,
    Empty,
}

fn tile_from_char(c: char) -> Option<Tile> {
    match c {
        '#' => Some(Tile::Wall),
        '.' => Some(Tile::Dot),
        ' ' => Some(Tile::Empty),
        'X' => Some(Tile::PowerUp),
        'H' => Some(Tile::House),
        _ => None,
    }
}

impl Map {
    pub fn new() -> Self {
        Map::default()
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || x >= self.width as i32 {
            None
        } else if y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(self.tiles[self.width * y as usize + x as usize])
        }
    }

    // TODO: rename this
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        match self.get(x, y) {
            Some(Tile::Wall) => true,
            Some(Tile::House) => true,
            _ => false,
        }
    }

    pub fn is_house(&self, x: i32, y: i32) -> bool {
        match self.get(x, y) {
            Some(Tile::House) => true,
            _ => false,
        }
    }

    fn set(&mut self, x: u32, y: u32, tile: Tile) {
        let (x, y) = (x as usize, y as usize);
        self.tiles[self.width * y + x] = tile;
    }

    pub fn consume(&mut self, x: i32, y: i32) {
        if let Some(Tile::Dot) = self.get(x, y) {
            self.pellets -= 1;
        };
        self.set(x as u32, y as u32, Tile::Empty);
    }

    pub fn scan_lines(&self) -> ScanLine<'_> {
        ScanLine {
            map: &self,
            line: 0,
        }
    }

    pub fn pellets(&self) -> u32 {
        self.pellets
    }

    pub fn reset(&mut self) {
        for (x, y) in self.pellet_coords.iter().cloned() {
            self.tiles[self.width * y + x] = Tile::Dot;
        }
        self.pellets = self.pellet_coords.len() as u32;
    }
}

impl Default for Map {
    fn default() -> Self {
        let map_width = MAP_STR[0].len();
        let map_height = MAP_STR.len();
        let tiles: Vec<Tile> = MAP_STR
            .iter()
            .flat_map(|x| x.chars())
            .filter_map(tile_from_char)
            .collect();
        assert_eq!(tiles.len(), map_width * map_height);
        let n_pellets = tiles
            .iter()
            .filter(|c| if let Tile::Dot = c { true } else { false })
            .count() as u32;
        Map {
            width: map_width,
            height: map_height,
            tiles,
            pellet_coords: pellet_coords(),
            pellets: n_pellets,
        }
    }
}

pub struct ScanLine<'a> {
    map: &'a Map,
    line: usize,
}

impl<'a> Iterator for ScanLine<'a> {
    type Item = &'a [Tile];

    fn next(&mut self) -> Option<&'a [Tile]> {
        let line_start = self.line * self.map.width;
        self.line += 1;
        if line_start >= self.map.tiles.len() {
            None
        } else {
            Some(&self.map.tiles[line_start..(line_start + self.map.width)])
        }
    }
}

// DEBUG
#[allow(dead_code)]
impl Map {
    pub fn remove_all_pellets(&mut self) {
        self.pellets = 0;
    }
}
