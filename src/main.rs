use text_io::read;
use core::fmt;
use std::fmt::Formatter;
use std::ops::Index;

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

struct GameMessage {
    contents: String
}

impl GameMessage {
    fn print(&self) {
        println!("{}", self.contents);
    }
}

struct Area {
    description: String,
    inventory: Inventory
}

impl Area {
    fn meadows() -> Area {
        Area {
            description: String::from("Your feet rest upon green meadows."),
            inventory: Inventory { items: vec![ Item { name: String::from("Potion") } ] }
        }
    }

    fn look(&self) -> GameMessage {
        let mut description = self.description.clone();
        description.push_str(&format!("\nYou look around, and see:\n{}", self.inventory));

        GameMessage { contents: description }
    }
}

#[derive(Clone)]
struct Item {
    name: String
}

struct Inventory {
    items: Vec<Item>
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { items: vec![] }
    }

    fn with (self, item: Item) -> Inventory {
        let mut new: Vec<Item> = self.items.clone();
        new.push(item);
        Inventory { items: new }
    }

    fn without(self, index: usize) -> Inventory {
        let mut new: Vec<Item> = self.items.clone();
        new.remove(index);
        Inventory { items: new }
    }

    fn look(&self) -> GameMessage {
        GameMessage { contents: format!("Your inventory:\n{}", self) }
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

enum Game {
    Running(GameState),
    NotRunning(String)
}

struct GameState {
    pub last_message: GameMessage,
    inventory: Inventory,
    area: Area
}

impl GameState {
    fn new() -> GameState {
        GameState {
            last_message: GameMessage { contents: String::new() },
            inventory: Inventory::new(),
            area: Area::meadows()
        }
    }

    fn process(self, input: String) -> Game {
        let inputs = input.split_whitespace().collect::<Vec<&str>>();
        match inputs[0] {
            "look" => {
                Game::Running(GameState { last_message: self.area.look(), ..self })
            }
            "pickup" => {
                let index = inputs[1].parse::<usize>().unwrap() - 1;
                let item = self.area.inventory[&index].clone();
                Game::Running(GameState {
                    last_message: GameMessage { contents: String::from(format!("You pickup the {}", item.name)) },
                    inventory: self.inventory.with(item),
                    area: Area { inventory: self.area.inventory.without(index), ..self.area },
                    ..self
                })
            }
            "inventory" => {
                Game::Running(GameState { last_message: self.inventory.look(), ..self })
            }
            "exit" => {
                Game::NotRunning(String::from("Ye hath not the faith to go on"))
            }
            "help" => {
                Game::Running(GameState { last_message: GameMessage { contents: String::from("Known commands are:\nlook\npickup\nexit\nhelp") }, ..self })
            }
            _ => {
                Game::Running(GameState { last_message: GameMessage{
                    contents: format!("Ye sepaketh nonsense, I know not the command '{}'. Try 'help'.", inputs[0])
                }, ..self })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GameState, Game};

    impl Game {
        fn assert_message(&self, expected: &str) {
            let expected = String::from(expected);
            match self {
                Game::Running(game_state) => {
                    assert_eq!(expected, game_state.last_message.contents);
                },
                Game::NotRunning(final_message) => {
                    assert_eq!(expected, final_message.to_owned());
                }
            }
        }
    }

    #[test]
    fn exit_changes_is_running_state_to_false() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("exit"));
        assert!(matches!(game, Game::NotRunning { .. }));
    }

    #[test]
    fn look_around() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("look"));
        game.assert_message("Your feet rest upon green meadows.\nYou look around, and see:\n1: Potion");
    }

    #[test]
    fn respond_helpfully_to_unknown_commands() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("gibberish"));
        game.assert_message("Ye sepaketh nonsense, I know not the command 'gibberish'. Try 'help'.");
    }

    #[test]
    fn pickup_item() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("pickup 1"));
        game.assert_message("You pickup the Potion");
        let game = match game {
            Game::Running(game_state) => {
                game_state.process(String::from("inventory"))
            }
            _ => panic!("Expected game to be running")
        };
        game.assert_message("Your inventory:\n1: Potion");
    }

    #[test]
    fn picked_up_items_no_longer_in_area() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("pickup 1"));
        let game = match game {
            Game::Running(game_state) => {
                game_state.process(String::from("look"))
            }
            _ => panic!("Expected game to be running")
        };
        game.assert_message("Your feet rest upon green meadows.\nYou look around, and see:\nNothing.");
    }

    #[test]
    fn print_help_Message() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("help"));
        game.assert_message("Known commands are:\nlook\npickup\nexit\nhelp");
    }
}
