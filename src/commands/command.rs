use crate::{Game, GameState};

pub trait Command {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game;
}
