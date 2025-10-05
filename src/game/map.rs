// const MAP_STR: [&'static str; 10] = [
//     "############################",
//     "#................X......#..#",
//     "#......#..............#.#..#",
//     "#......#....######....#.#..#",
//     "#...####..............#.#..#",
//     "#......#.......X.X....#.#..#",
//     "#......###########....#....#",
//     "#......#..............#....#",
//     "#.....................#....#",
//     "############################",
// ];

fn pellet_coords(map_str: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    map_str
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '.')
                .map(move |(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .fold(vec![], |mut acc, mut l| {
            acc.append(&mut l);
            acc
        })
}

// Introduce 2d array type to simplify this code?
// fn ghost_coords(map_str: &Vec<Vec<char>>, marker: char) -> (i32, i32) {
//     let mut coords = map_str.iter().enumerate().map(|(y, line)| {
//         line.iter()
//             .enumerate()
//             .filter(|(_, c)| **c == marker)
//             .map(move |(x, _)| (x, y))
//             .collect::<Vec<(usize, usize)>>()
//     });
//     if coords.len() == 1 {
//         let c = coords.next().unwrap()[0];
//         return (c.0 as i32, c.1 as i32);
//     } else {
//         eprintln!("coords: {:?} for marker; {}", coords, marker);
//         panic!("Expected exactly one ghotst of this type.");
//     }
// }

pub struct Map {
    pub width: usize,
    pub height: usize,
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
    pub fn new(map_str: Vec<Vec<char>>) -> Self {
        let map_width = map_str[0].len();
        let map_height = map_str.len();
        let pellet_coords = pellet_coords(&map_str);
        let tiles: Vec<Tile> = map_str
            .into_iter()
            .flatten()
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
            pellet_coords,
            pellets: n_pellets,
        }
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
