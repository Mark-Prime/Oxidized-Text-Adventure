use std::io::{stdout};
use image::{io::Reader as ImageReader, Pixel};
use std::fmt::{self, Display, Formatter};
use crossterm::{
    execute,
    cursor::{MoveTo},
    terminal::{Clear, ClearType},
};

pub struct World {
    tiles: Box<[Box<[Tile]>]>,
    max_x: u8,
}

impl World {
    pub fn new(src: String) -> World {
        let img = ImageReader::open(src).unwrap().decode().unwrap();
        let mut tiles = World::default().tiles;
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

        print!("┌");

        for _ in 0..self.max_x {
            print!("─");
        }

        println!("┐");
        for row_y in self.tiles.iter().enumerate() {
            let (i_y, row) = row_y;
            print!("│");
            for column_x in row.iter().enumerate() {
                let (i_x, tile) = column_x;
                execute!(
                    stdout(),
                    MoveTo((i_x + 1).try_into().unwrap(), (i_y + 1).try_into().unwrap())
                ).unwrap();
                if tile.discovered | true {
                    print!("{}", tile.name)
                }
            }
            println!("│");
        }

        print!("└");

        for _ in 0..self.max_x {
            print!("─");
        }
        println!("┘");
    }

    pub fn discover(&mut self, x:usize, y:usize, display:bool) {
        self.tiles[y][x].discovered = true;

        if !display {
            self.tiles[y+1][x].discovered = true;
            self.tiles[y-1][x].discovered = true;
            self.tiles[y][x+1].discovered = true;
            self.tiles[y][x-1].discovered = true;
            return;
        }

        use TileName::*;
        let msg = match self.tiles[y][x].name {
            Sand => "You are standing on wet sand.",
            Plains => "You see a grassy field.",
            Trees => "You see a few trees scattered around.",
            Forest => "You see trees all around you.",
            Hills => "You see rolling hills.",
            _ => "How did you manage to get here?",
        };

        println!("{}", msg);

        if self.tiles[y+1][x].discovered != true {
            self.tiles[y+1][x].discovered = true;

            use TileName::*;
            let msg = match self.tiles[y+1][x].name {
                Ocean => "You see the ocean",
                Water => "You see running water",
                Sand => "You see wet sand",
                Plains => "You see a grassy field",
                Trees => "You see a few trees",
                Forest => "You see a forest",
                Hills => "You see rolling hills",
                Mountain => "You see a mountain",
                Town => "You see a town",
            };

            println!("{} to the North", msg);
        }

        if self.tiles[y-1][x].discovered != true {
            self.tiles[y-1][x].discovered = true;

            use TileName::*;
            let msg = match self.tiles[y-1][x].name {
                Ocean => "You see the ocean",
                Water => "You see running water",
                Sand => "You see wet sand",
                Plains => "You see a grassy field",
                Trees => "You see a few trees",
                Forest => "You see a forest",
                Hills => "You see rolling hills",
                Mountain => "You see a mountain",
                Town => "You see a town",
            };

            println!("{} to the South.", msg);
        }

        if self.tiles[y][x+1].discovered != true {
            self.tiles[y][x+1].discovered = true;

            use TileName::*;
            let msg = match self.tiles[y][x+1].name {
                Ocean => "You see the ocean",
                Water => "You see running water",
                Sand => "You see wet sand",
                Plains => "You see a grassy field",
                Trees => "You see a few trees",
                Forest => "You see a forest",
                Hills => "You see rolling hills",
                Mountain => "You see a mountain",
                Town => "You see a town",
            };

            println!("{} to the East", msg);
        }

        if self.tiles[y][x-1].discovered != true {
            self.tiles[y][x-1].discovered = true;

            use TileName::*;
            let msg = match self.tiles[y][x-1].name {
                Ocean => "You see the ocean",
                Water => "You see running water",
                Sand => "You see wet sand",
                Plains => "You see a grassy field",
                Trees => "You see a few trees",
                Forest => "You see a forest",
                Hills => "You see rolling hills",
                Mountain => "You see a mountain",
                Town => "You see a town",
            };

            println!("{} to the West", msg);
        }

        return;
    }

    pub fn display_tile(&self, x: usize, y: usize) {
        use TileName::*;
        let msg = match self.tiles[x][y].name {
            Ocean => "You see the ocean",
            Water => "You see running water",
            Sand => "You see wet sand",
            Plains => "You see a grassy field",
            Trees => "You see a few trees",
            Forest => "You see a forest",
            Hills => "You see rolling hills",
            Mountain => "You see a mountain",
            Town => "You see a town",
        };

        println!("{}.", msg);
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
            tiles: vec![vec![Tile::default(); 200].into_boxed_slice(); 50].into_boxed_slice(),
            max_x: 200,
        };
    }
}

#[derive(Clone)]
enum TileName {
    Ocean,
    Water,
    Sand,
    Plains,
    Trees,
    Forest,
    Hills,
    Mountain,
    Town,
    // Ferry,
    // Castle
}

impl Display for TileName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use TileName::*;
        match self {
            Ocean => write!(f, "▓"),
            Water => write!(f, "▒"),
            Sand => write!(f, "░"),
            Plains => write!(f, "."),
            Trees => write!(f, "⭡"),
            Forest => write!(f, "⯭"),
            Hills => write!(f, "⏶"),
            Mountain => write!(f, "◮"),
            Town => write!(f, "⌂"),
            // Ferry => write!(f, "⛴"),
            // Castle => write!(f, "🏰"), //⛫
        }
    }
}

#[derive(Clone)]
struct Tile {
    name: TileName,
    accessible: bool,
    discovered: bool,
    town: Option<Town>,
}

impl Tile {
    fn new(rgb: (u8, u8, u8)) -> Tile {
        match rgb {
            (100, 75, 0) => {
                return Tile {
                    name: TileName::Mountain, // Mountain
                    accessible: false,
                    discovered: false,
                    town: None,
                };
            },
            (255, 191, 0) => {
                return Tile {
                    name: TileName::Hills, // hills
                    accessible: true,
                    discovered: false,
                    town: None,
                };
            },
            (0, 255, 0) => {
                return Tile {
                    name: TileName::Plains, // deep forest
                    accessible: true,
                    discovered: false,
                    town: None,
                };
            },
            (0, 200, 0) => {
                return Tile {
                    name: TileName::Trees, // trees
                    accessible: true,
                    discovered: false,
                    town: None,
                };
            },
            (0, 100, 0) => {
                return Tile {
                    name: TileName::Forest, // plains
                    accessible: true,
                    discovered: false,
                    town: None,
                };
            },
            (100, 100, 255) => {
                return Tile {
                    name: TileName::Sand, // lake/shallow water/sand
                    accessible: false,
                    discovered: false,
                    town: None,
                };
            },
            (0, 0, 200) => {
                return Tile {
                    name: TileName::Water, // deep water
                    accessible: false,
                    discovered: false,
                    town: None,
                };
            },
            (0, 0, 100) => {
                return Tile {
                    name: TileName::Ocean, // ocean
                    accessible: false,
                    discovered: false,
                    town: None,
                };
            },
            (255, 255, 255) => {
                return Tile {
                    name: TileName::Town, // ocean
                    accessible: true,
                    discovered: false,
                    town: None,
                };
            },
            _ => {
                return Tile::default();
            }
        }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            name: TileName::Plains,
            accessible: true,
            discovered: false,
            town: None
        }
    }
}

#[derive(Copy, Clone)]
enum TownSize {
    Capital,
    City,
    Village,
    Hut,
}

#[derive(Clone)]
struct Town {
    name: String,
    size: TownSize,
    quests: Vec<Quest>,
}

#[derive(Clone)]
struct Quest {
    name: String,
    unlocked: bool,
    completed: bool,
}