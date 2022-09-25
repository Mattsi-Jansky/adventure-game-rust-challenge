use core::fmt;
use std::fmt::Formatter;
use std::ops::Index;

use text_io::read;

use crate::area::Area;
use crate::inventory::{Inventory, Item};
use crate::game::{Game, GameState};

mod inventory;
mod area;
pub mod commands;
mod game;

fn main() {
    let mut game = Game::Running(GameState::new());
    loop {
        match game {
            Game::Running(game_state) => {
                game_state.last_message.print();
                println!("Please enter a command:");
                let line: String = read!("{}\n");
                game = game_state.process(line);
            }
            Game::NotRunning(final_message) => {
                println!("{}", final_message);
                break;
            }
        }
    }
}
