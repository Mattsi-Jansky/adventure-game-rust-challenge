use crate::inventory::{Inventory};
use crate::game::GameMessage;
use crate::consumable::{Consumable, Potion};

pub struct Area<T: Consumable> {
    description: String,
    inventory: Inventory<T>
}

impl<T: Consumable> Area<T> {
    pub fn meadows() -> Area<T> {
        Area {
            description: String::from("Your feet rest upon green meadows."),
            inventory: Inventory::from(vec![ Potion {} ])
        }
    }

    pub fn look(&self) -> GameMessage {
        let mut description = self.description.clone();
        description.push_str(&format!("\nYou look around, and see:\n{}", self.inventory));

        GameMessage::new(&description[..])
    }

    pub fn without_item(self, index: usize) -> Area<T> {
        Area { inventory: self.inventory.without(index), ..self }
    }

    pub fn get_from_inventory(&self, index: &usize) -> T {
        self.inventory[index].clone()
    }
}
