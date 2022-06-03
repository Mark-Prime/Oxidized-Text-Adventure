use core::panic;
use std::io::{stdin, stdout};
use crossterm::{
    execute,
    cursor,
    terminal::{Clear, ClearType}
};

mod player;
mod item;
mod world;

fn cls() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();
}

fn main() {
    cls();
    let world = world::World::new("./src/img/text-map.png".to_string());
    println!("Input your name:");

    let mut user_name = String::new();
    stdin().read_line(&mut user_name).expect("Invalid Name");

    let mut player = match player::Player::new(user_name, world) {
        Ok(ply) => ply,
        Err(_) => panic!("AAAA")
    };

    cls();

    let (x, y) = player.get_pos();

    player.world.discover(x as usize, y as usize, true);
    loop {
        let mut cmd = String::new();
        stdin().read_line(&mut cmd).expect("Invalid Command");

        cls();

        match player.check_command(cmd.to_string()) {
            Ok(player) => player,
            Err(err) => {
                println!("{}", err);
            }
        };
    }
}
