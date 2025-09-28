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
    tiles: [Tile; (MAP_WIDTH * MAP_HEIGHT) as usize],
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
        if x < 0 || x >= MAP_WIDTH as i32 {
            None
        } else if y < 0 || y >= MAP_HEIGHT as i32 {
            None
        } else {
            Some(self.tiles[MAP_WIDTH * y as usize + x as usize])
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
        self.tiles[MAP_WIDTH * y + x] = tile;
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
            self.tiles[MAP_WIDTH * y + x] = Tile::Dot;
        }
        self.pellets = self.pellet_coords.len() as u32;
    }
}

impl Default for Map {
    fn default() -> Self {
        let map: Vec<Tile> = MAP_STR
            .iter()
            .flat_map(|x| x.chars())
            .filter_map(tile_from_char)
            .collect();
        assert_eq!(map.len(), MAP_WIDTH * MAP_HEIGHT);
        let mut m = [Tile::Empty; MAP_WIDTH * MAP_HEIGHT];
        m.copy_from_slice(&map);
        for i in 0..map.len() {
            m[i] = map[i];
        }
        let n_pellets = map
            .iter()
            .filter(|c| if let Tile::Dot = c { true } else { false })
            .count() as u32;
        Map {
            tiles: m,
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
        let line_start = self.line * MAP_WIDTH;
        self.line += 1;
        if line_start >= self.map.tiles.len() {
            None
        } else {
            Some(&self.map.tiles[line_start..(line_start + MAP_WIDTH)])
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
