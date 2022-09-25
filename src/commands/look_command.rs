use crate::commands::command::*;
use crate::{Game, GameState};

pub struct LookCommand {}

impl Command for LookCommand {
    fn execute(&self, game_state: GameState) -> Game {
        Game::Running(GameState {
            last_message: game_state.area.look(),
            ..game_state
        })
    }
}
