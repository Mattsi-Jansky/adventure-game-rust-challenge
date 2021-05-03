use crate::inventory::{Inventory, Item, ItemType};
use crate::game::GameMessage;

pub struct Area {
    description: String,
    inventory: Inventory
}

impl Area {
    pub fn meadows() -> Area {
        Area {
            description: String::from("Your feet rest upon green meadows."),
            inventory: Inventory::from(vec![ Item { name: String::from("Potion"), item_type: ItemType::Potion } ])
        }
    }

    pub fn look(&self) -> GameMessage {
        let mut description = self.description.clone();
        description.push_str(&format!("\nYou look around, and see:\n{}", self.inventory));

        GameMessage::new(&description[..])
    }

    pub fn without_item(self, index: usize) -> Area {
        Area { inventory: self.inventory.without(index), ..self }
    }

    pub fn get_from_inventory(&self, index: &usize) -> Item {
        self.inventory[index].clone()
    }
}
