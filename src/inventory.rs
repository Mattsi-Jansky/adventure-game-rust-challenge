use core::fmt;
use core::fmt::Formatter;
use crate::game::GameMessage;
use std::ops::Index;
use crate::consumable::Consumable;

pub struct Inventory<T: Consumable> {
    items: Vec<T>
}

impl<T: Consumable> Inventory<T> {
    pub fn new() -> Inventory<T> {
        Inventory { items: vec![] }
    }

    pub fn from(items: Vec<T>) -> Inventory<T> {
        Inventory { items }
    }

    pub fn with (self, item: T) -> Inventory<T> {
        let mut new: Vec<dyn Consumable> = self.items.clone();
        new.push(item);
        Inventory { items: new }
    }

    pub fn without(self, index: usize) -> Inventory<T> {
        let mut new: Vec<dyn Consumable> = self.items.clone();
        new.remove(index);
        Inventory { items: new }
    }

    pub fn look(&self) -> GameMessage {
        GameMessage::new(&format!("Your inventory:\n{}", self)[..])
    }
}

impl<T: Consumable> fmt::Display for Inventory<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
               &self.items.iter().enumerate()
                .map(|(i, item)| format!("{}: {}", i + 1, item.name))
                .reduce(|a, b| format!("{}\n{}", a, b))
                .unwrap_or(String::from("Nothing."))[..]
        )
    }
}

impl<T: Consumable> Index<&'_ usize> for Inventory<T> {
    type Output = T;

    fn index(&self, index: &usize) -> &T {
        let index = index.clone();
        &self.items[index]
    }
}
