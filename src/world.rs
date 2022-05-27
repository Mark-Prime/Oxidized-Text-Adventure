use std::io::{stdout};
use image::{io::Reader as ImageReader, Pixel};
use crossterm::{
    execute,
    cursor::{MoveTo},
    terminal::{Clear, ClearType},
};

pub struct World {
    tiles: [[Tile; 200]; 50],
    max_x: u8,
}

impl World {
    pub fn new(src: String) -> World {
        let img = ImageReader::open(src).unwrap().decode().unwrap();
        let mut tiles = [[Tile::default(); 200]; 50];
        let img = img.into_rgb8();
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let rgb = img.get_pixel(x, y).channels();
                tiles[y as usize][x as usize] = Tile::new((rgb[0], rgb[1], rgb[2]));
            };
        };
        
        return World {
            tiles: tiles,
            max_x: 200,
        };
    }

    pub fn print_map(&self) {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();

        let key = ["▓", "▒", "░", ".", "f", "F", "m", "▲"];
        for _ in 0..self.max_x {
            print!("|");
        }
        println!("|");
        for row_y in self.tiles.iter().enumerate() {
            let (i_y, row) = row_y;
            print!("|");
            for column_x in row.iter().enumerate() {
                let (i_x, tile) = column_x;
                execute!(
                    stdout(),
                    MoveTo((i_x + 1).try_into().unwrap(), (i_y + 1).try_into().unwrap())
                ).unwrap();
                if tile.discovered | true {
                    print!("{}", key[tile.name as usize])
                }
            }
            println!("|");
        }
        for _ in 0..self.max_x {
            print!("|");
        }
        println!("|");
    }

    pub fn discover(&mut self, x:usize, y:usize) {
        self.tiles[y][x].discovered = true;
    }

    pub fn is_accessible(&self, x:usize, y:usize) -> bool {
        if self.tiles[y][x].accessible {
            return true;
        }
        return false;
    }
}

impl Default for World {
    fn default() -> World {
        return World{
            tiles: [[Tile::default(); 200]; 50],
            max_x: 200,
        };
    }
}

#[derive(Copy, Clone)]
struct Tile {
    name: i32,
    accessible: bool,
    discovered: bool,
}

impl Tile {
    fn new(rgb: (u8, u8, u8)) -> Tile {
        match rgb {
            (100, 75, 0) => {
                return Tile {
                    name: 7, // Mountain
                    accessible: false,
                    discovered: false,
                };
            },
            (255, 191, 0) => {
                return Tile {
                    name: 6, // hills
                    accessible: true,
                    discovered: false,
                };
            },
            (0, 255, 0) => {
                return Tile {
                    name: 3, // deep forest
                    accessible: true,
                    discovered: false,
                };
            },
            (0, 200, 0) => {
                return Tile {
                    name: 4, // trees
                    accessible: true,
                    discovered: false,
                };
            },
            (0, 100, 0) => {
                return Tile {
                    name: 5, // plains
                    accessible: true,
                    discovered: false,
                };
            },
            (100, 100, 255) => {
                return Tile {
                    name: 2, // lake/shallow water
                    accessible: false,
                    discovered: false,
                };
            },
            (0, 0, 200) => {
                return Tile {
                    name: 1, // deep water
                    accessible: false,
                    discovered: false,
                };
            },
            (0, 0, 100) => {
                return Tile {
                    name: 0, // ocean
                    accessible: false,
                    discovered: false,
                };
            },
            _ => {
                return Tile::default();
            }
        }
    }
}

impl Default for Tile {
    //  let key = ["O", "W", "L", "P", "f", "F", "H", "M"];
    fn default() -> Tile {
        Tile { name: 1, accessible: true, discovered: false}
    }
}