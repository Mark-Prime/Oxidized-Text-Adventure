// use std::error::Error;
use std::io::stdout;
use std::fmt::{self, Display, Formatter};
use crossterm::{
    execute,
    cursor::{self, MoveTo}};

use crate::item::Item;
use crate::world::World;

pub enum PlayerError {
    NoName,
    InvalidMovement,
    InvalidRotation,
    InvalidCommandLength,
    UnknownCommand,
    OutOfBounds
}

// impl Error for PlayerError {}

impl Display for PlayerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use PlayerError::*;
        match self {
            NoName => write!(f, "no player name"),
            InvalidMovement => write!(f, "invalid movement input"),
            InvalidRotation => write!(f, "invalid direction change"),
            InvalidCommandLength => write!(f, "Command too short"),
            OutOfBounds => write!(f, "The movement would put you out of bounds"),
            UnknownCommand => write!(f, "This command does not exist"),
        }
    }
}

pub struct Player {
    name: String,
    health: u8,
    armor: u8,
    pos: (i32, i32),
    pub facing: u8,
    inventory: Vec<Item>,
    pub world: World
}

impl Player {
    pub fn new(name: String, world: World) -> Result<Player, PlayerError> {
        if name.len() > 0 {
            return Ok(Player {
                name: name,
                health: 100,
                armor: 0,
                pos: (164, 40),
                facing: 0,
                inventory: vec!(Item::new("Compass", "Always points the way home").unwrap(), Item::new("Map", "The World Map").unwrap()),
                world: world
            })
        }

        return Err(PlayerError::NoName)
    }

    pub fn get_pos(&self) -> (i32, i32){
        self.pos
    }

    pub fn move_facing(&mut self, direction: &str) -> Result<(), PlayerError> {
        self.rotate(direction)?;
        match self.facing {
            0 => { // North
                if self.pos.1 < 100 {
                    self.pos.1 -= 1;
                    self.world.discover(self.pos.0.try_into().unwrap(), self.pos.1.try_into().unwrap(), true);
                    return Ok(());
                }
                return Err(PlayerError::OutOfBounds);
            }
            1 => { // East
                if self.pos.0 < 100 {
                    self.pos.0 += 1;
                    self.world.discover(self.pos.0.try_into().unwrap(), self.pos.1.try_into().unwrap(), true);
                    return Ok(());
                }
                return Err(PlayerError::OutOfBounds);
            }
            2 => { // South
                if self.pos.1 > 0 {
                    if self.world.is_accessible(self.pos.0.try_into().unwrap(), (self.pos.1 + 1 ).try_into().unwrap()) {
                        self.pos.1 += 1;
                        self.world.discover(self.pos.0.try_into().unwrap(), self.pos.1.try_into().unwrap(), true);
                        return Ok(());
                    }
                    return Err(PlayerError::OutOfBounds);
                }
                return Err(PlayerError::OutOfBounds);
            }
            3 => { // West
                if self.pos.0 > 0 {
                    self.pos.0 -= 1;
                    self.world.discover(self.pos.0.try_into().unwrap(), self.pos.1.try_into().unwrap(), true);
                    return Ok(());
                }
                return Err(PlayerError::OutOfBounds);
            }
            _ => {
                return Err(PlayerError::InvalidMovement);
            }
        }
    }

    fn rotate(&mut self, direction: &str) -> Result<(), PlayerError> {
        match direction {
            "forward" | "forwards" | "up" => {
                return Ok(());
            },
            "backward" | "backwards" | "back" | "down" => {
                self.facing += 2;
                if self.facing > 3 {
                    self.facing -= 4;
                }
                Ok(())
            },
            "left" => {
                if self.facing == 0 {
                    self.facing = 3;
                } else {
                    self.facing -= 1;
                }
                Ok(())
            },
            "right" => {
                if self.facing == 3 {
                    self.facing = 0;
                } else {
                    self.facing += 1;
                }
                Ok(())
            },
            _ => {
                Err(PlayerError::InvalidRotation)
            }
        }
    }

    pub fn display_inventory(&self) {
        let mut inventory = self.inventory.iter();
        loop {
            let item = inventory.next();
            if item.is_none() {
                break;
            }

            print!("{}", item.unwrap().name);
            execute!(stdout(), cursor::MoveToColumn(15)).expect("failed to execute");
            println!("{}", item.unwrap().describe());
        }
    }

    pub fn check_command(&mut self, cmd: String) -> Result<(), PlayerError>{
        if cmd.len() < 4 {
            return Err(PlayerError::InvalidCommandLength);
        }
        let mut cmd_args = cmd.split_whitespace();
    
        let base_cmd = cmd_args.next().unwrap();
    
        match base_cmd {
            "help" => {
                println!("Avalible commands:\nmove <forward/backward/left/right>\ninventory\nstatus");
                Ok(())
            },
            "inventory" => {
                self.display_inventory();
                Ok(())
            },
            "status" => {
                println!("Name: {}", self.name);
                println!("Health: {}", self.health);
                println!("Armor: {}", self.armor);
                Ok(())
            },
            "use" => {
                let item = cmd_args.next();
                if item.is_none() {return Err(PlayerError::InvalidCommandLength)}
                let item = item.unwrap();
                match item {
                    "map" => {
                        self.world.print_map();
                        execute!(
                            stdout(),
                            MoveTo((self.pos.0 + 1).try_into().unwrap(), (self.pos.1 + 1).try_into().unwrap())
                        ).unwrap();
                        print!("☺");
                        execute!(
                            stdout(),
                            MoveTo((0).try_into().unwrap(), (52).try_into().unwrap())
                        ).unwrap();
                        return Ok(());
                    },
                    "compass" => {
                        let key = ["North", "South", "East", "West"];
                        println!("You are facing {}", key[self.facing as usize]);
                        return Ok(());
                    },
                    _ => {return Err(PlayerError::InvalidMovement);}
                }
            }
            "look" => {
                let direction = cmd_args.next();
                if direction.is_none() {return Err(PlayerError::InvalidCommandLength)}
                let direction = direction.unwrap();
    
                if direction != "forward"
                        && direction != "forwards"
                        && direction != "up"
                        && direction != "backward"
                        && direction != "backwards"
                        && direction != "down"
                        && direction != "back"
                        && direction != "left"
                        && direction != "right" {
                    return Err(PlayerError::InvalidMovement);
                }
                match self.rotate(direction) {
                    Err(_) => return Err(PlayerError::InvalidMovement),
                    Ok(_) => {
                        match self.facing {
                            0 => {
                                self.world.display_tile(self.pos.0.try_into().unwrap(), (self.pos.1 - 1).try_into().unwrap());
                                return Ok(());
                            },
                            1 => {
                                self.world.display_tile((self.pos.0 - 1).try_into().unwrap(), (self.pos.1).try_into().unwrap());
                                return Ok(());
                            },
                            2 => {
                                self.world.display_tile(self.pos.0.try_into().unwrap(), (self.pos.1 + 1).try_into().unwrap());
                                return Ok(());
                            },
                            3 => {
                                self.world.display_tile((self.pos.0 + 1).try_into().unwrap(), (self.pos.1).try_into().unwrap());
                                return Ok(());
                            },
                            _ => {
                                return Err(PlayerError::InvalidMovement);
                            }
                        }
                    },
                };
            },
            "move" => {
                let direction = cmd_args.next();
                if direction.is_none() {return Err(PlayerError::InvalidCommandLength)}
                let direction = direction.unwrap();
    
                if direction != "forward"
                        && direction != "forwards"
                        && direction != "up"
                        && direction != "backward"
                        && direction != "backwards"
                        && direction != "down"
                        && direction != "back"
                        && direction != "left"
                        && direction != "right" {
                    
                    return Err(PlayerError::InvalidMovement);
                }
    
                self.move_facing(direction)
            },
            _ => {return Err(PlayerError::UnknownCommand)},
        }
    }
}