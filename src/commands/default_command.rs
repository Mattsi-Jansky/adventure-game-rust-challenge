use crate::commands::command::*;
use crate::game::GameMessage;
use crate::{Game, GameState};

pub struct DefaultCommand {}

impl Command for DefaultCommand {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game {
        Game::Running(GameState {
            last_message: GameMessage {
                contents: format!(
                    "Ye sepaketh nonsense, I know not the command '{}'. Try 'help'.",
                    inputs[0]
                ),
            },
            ..game_state
        })
    }
}
