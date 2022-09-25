use crate::commands::command::*;
use crate::game::GameMessage;
use crate::inventory::ItemType;
use crate::{Game, GameState};

pub struct UseCommand {}

impl Command for UseCommand {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game {
        let index = inputs[1].parse::<usize>().unwrap() - 1;
        let item = game_state.inventory[&index].clone();
        match item.item_type {
            ItemType::Potion => Game::Running(GameState {
                last_message: GameMessage {
                    contents: String::from("You drink the Potion."),
                },
                inventory: game_state.inventory.without(index),
                health: game_state.health + 1,
                ..game_state
            }),
            ItemType::Venom => Game::Running(GameState {
                last_message: GameMessage {
                    contents: String::from("You drink the venom (For some reason???)."),
                },
                inventory: game_state.inventory.without(index),
                health: game_state.health - 1,
                ..game_state
            }),
        }
    }
}
