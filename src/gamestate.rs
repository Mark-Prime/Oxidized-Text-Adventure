use crate::player::Player;
use crate::world::World;

#[derive(Debug)]
pub(crate) struct GameState {
    world: Option<World>,
    player: Option<Player>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState { world: None, player: None }
    }

    // getters
    pub fn world(&self) -> &World {
        self.world.as_ref().unwrap()
    }

    pub fn player(&self) -> &Player {
        self.player.as_ref().unwrap()
    }

    pub fn world_mut(&mut self) -> &mut World {
        self.world.as_mut().unwrap()
    }

    pub fn player_mut(&mut self) -> &mut Player {
        self.player.as_mut().unwrap()
    }

}