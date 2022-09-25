use crate::commands::command::*;
use crate::{Game, GameState};

pub struct ExitCommand {}

impl Command for ExitCommand {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game {
        Game::NotRunning(String::from("Ye hath not the faith to go on"))
    }
}
