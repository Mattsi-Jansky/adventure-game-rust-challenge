use crate::commands::default_command::DefaultCommand;
use crate::commands::exit_command::ExitCommand;
use crate::commands::help_command::HelpCommand;
use crate::commands::inventory_command::InventoryCommand;
use crate::commands::look_command::LookCommand;
use crate::commands::pickup_command::PickupCommand;
use crate::commands::status_command::StatusCommand;
use crate::commands::use_command::UseCommand;
use crate::{Game, GameState};

pub trait Command {
    fn execute(&self, game_state: GameState, inputs: Vec<&str>) -> Game;
}

pub fn select_command(input: &str) -> Box<dyn Command> {
    match input {
        "look" => Box::new(LookCommand {}),
        "pickup" => Box::new(PickupCommand {}),
        "inventory" => Box::new(InventoryCommand {}),
        "status" => Box::new(StatusCommand {}),
        "use" => Box::new(UseCommand {}),
        "exit" => Box::new(ExitCommand {}),
        "help" => Box::new(HelpCommand {}),
        _ => Box::new(DefaultCommand {}),
    }
}
