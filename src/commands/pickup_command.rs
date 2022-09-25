use crate::commands::command::*;
use crate::game::GameMessage;
use crate::{Game, GameState};

pub struct PickupCommand {}

impl Command for PickupCommand {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game {
        let index = inputs[1].parse::<usize>().unwrap() - 1;
        let item = game_state.area.get_from_inventory(&index);
        Game::Running(GameState {
            last_message: GameMessage {
                contents: String::from(format!("You pickup the {}", item.name)),
            },
            inventory: game_state.inventory.with(item),
            area: game_state.area.without_item(index),
            ..game_state
        })
    }
}
