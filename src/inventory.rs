use core::fmt;
use core::fmt::Formatter;
use crate::game::GameMessage;
use std::ops::Index;

#[derive(Clone)]
pub enum ItemType {
    Potion,
    Venom
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType
}

pub struct Inventory {
    items: Vec<Item>
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory { items: vec![] }
    }

    pub fn from(items: Vec<Item>) -> Inventory {
        Inventory { items }
    }

    pub fn with (self, item: Item) -> Inventory {
        let mut new: Vec<Item> = self.items.clone();
        new.push(item);
        Inventory { items: new }
    }

    pub fn without(self, index: usize) -> Inventory {
        let mut new: Vec<Item> = self.items.clone();
        new.remove(index);
        Inventory { items: new }
    }

    pub fn look(&self) -> GameMessage {
        GameMessage::new(&format!("Your inventory:\n{}", self)[..])
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
               &self.items.iter().enumerate()
                .map(|(i, item)| format!("{}: {}", i + 1, item.name))
                .reduce(|a, b| format!("{}\n{}", a, b))
                .unwrap_or(String::from("Nothing."))[..]
        )
    }
}

impl Index<&'_ usize> for Inventory {
    type Output = Item;

    fn index(&self, index: &usize) -> &Item {
        let index = index.clone();
        &self.items[index]
    }
}
