use crate::commands::command::*;
use crate::game::GameMessage;
use crate::{Game, GameState};

pub struct StatusCommand {}

impl Command for StatusCommand {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game {
        Game::Running(GameState {
            last_message: GameMessage {
                contents: format!("Level:{}\nHealth:{}", 1, game_state.health),
            },
            ..game_state
        })
    }
}
