use crate::commands::command::*;
use crate::game::GameMessage;
use crate::{Game, GameState};

pub struct HelpCommand {}

impl Command for HelpCommand {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game {
        Game::Running(GameState {
            last_message: GameMessage {
                contents: String::from(
                    "Known commands are:\nlook\npickup\ninventory\nuse\nstatus\nexit\nhelp",
                ),
            },
            ..game_state
        })
    }
}
