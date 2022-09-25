use crate::commands::command::*;
use crate::{Game, GameState};

pub struct InventoryCommand {}

impl Command for InventoryCommand {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game {
        Game::Running(GameState {
            last_message: game_state.inventory.look(),
            ..game_state
        })
    }
}
