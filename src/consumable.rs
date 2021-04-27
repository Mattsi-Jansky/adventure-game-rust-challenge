use crate::game::GameState;
use crate::player::Player;

pub trait Consumable {
    fn Use(&self, game_state: Player) -> Player;
    fn Name(&self) -> String;
}

pub struct Potion {  }

impl Consumable for Potion {
    fn Use(&self, player: Player) -> Player {
        Player {
            health: player.health + 1,
            ..player
        }
    }

    fn Name(&self) -> String{
        String::from("Potion")
    }
}