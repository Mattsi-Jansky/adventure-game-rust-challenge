use crate::inventory::{Inventory, Item, ItemType};
use crate::game::GameMessage;

pub struct Area {
    description: String,
    inventory: Inventory
}

impl Area {
    pub(crate) fn new(description: &str, inventory: Vec<Item>) -> Area {
        Area {
            description: String::from(description),
            inventory: Inventory::from(inventory)
        }
    }

    pub fn meadows() -> Area {
        Area {
            description: String::from("Your feet rest upon green meadows."),
            inventory: Inventory::from(vec![
                Item { name: String::from("Potion"), item_type: ItemType::Potion }
            ])
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_new_area() {
        let result = Area::new("my test area", vec![
            Item { name: String::from("my test item"), item_type: ItemType::Potion }
        ]);

        assert_eq!(result.description, "my test area");
        assert_eq!(result.inventory[&0].name, "my test item");
    }
}
