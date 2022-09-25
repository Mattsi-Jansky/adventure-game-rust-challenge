use crate::area::Area;
use crate::inventory::{Inventory, ItemType};

pub struct GameMessage {
    contents: String,
}

impl GameMessage {
    pub(crate) fn new(contents: &str) -> GameMessage {
        GameMessage {
            contents: String::from(contents),
        }
    }

    pub(crate) fn print(&self) {
        println!("{}", self.contents);
    }
}

pub enum Game {
    Running(GameState),
    NotRunning(String),
}

pub struct GameState {
    pub last_message: GameMessage,
    inventory: Inventory,
    area: Area,
    health: usize,
}

impl GameState {
    pub(crate) fn new() -> GameState {
        GameState {
            last_message: GameMessage {
                contents: String::new(),
            },
            inventory: Inventory::new(),
            area: Area::meadows(),
            health: 10,
        }
    }

    pub(crate) fn from(area: Area) -> GameState {
        GameState {
            last_message: GameMessage {
                contents: String::new(),
            },
            inventory: Inventory::new(),
            area,
            health: 10,
        }
    }

    pub(crate) fn process(self, input: String) -> Game {
        let inputs = input.split_whitespace().collect::<Vec<&str>>();
        match inputs[0] {
            "look" => Game::Running(GameState {
                last_message: self.area.look(),
                ..self
            }),
            "pickup" => {
                let index = inputs[1].parse::<usize>().unwrap() - 1;
                let item = self.area.get_from_inventory(&index);
                Game::Running(GameState {
                    last_message: GameMessage {
                        contents: String::from(format!("You pickup the {}", item.name)),
                    },
                    inventory: self.inventory.with(item),
                    area: self.area.without_item(index),
                    ..self
                })
            }
            "inventory" => Game::Running(GameState {
                last_message: self.inventory.look(),
                ..self
            }),
            "status" => Game::Running(GameState {
                last_message: GameMessage {
                    contents: format!("Level:{}\nHealth:{}", 1, self.health),
                },
                ..self
            }),
            "use" => {
                let index = inputs[1].parse::<usize>().unwrap() - 1;
                let item = self.inventory[&index].clone();
                match item.item_type {
                    ItemType::Potion => Game::Running(GameState {
                        last_message: GameMessage {
                            contents: String::from("You drink the Potion."),
                        },
                        inventory: self.inventory.with(item),
                        health: self.health + 1,
                        ..self
                    }),
                    ItemType::Venom => Game::Running(GameState {
                        last_message: GameMessage {
                            contents: String::from("You drink the venom (For some reason???)."),
                        },
                        inventory: self.inventory.with(item),
                        health: self.health - 1,
                        ..self
                    }),
                }
            }
            "exit" => Game::NotRunning(String::from("Ye hath not the faith to go on")),
            "help" => Game::Running(GameState {
                last_message: GameMessage {
                    contents: String::from(
                        "Known commands are:\nlook\npickup\ninventory\nuse\nstatus\nexit\nhelp",
                    ),
                },
                ..self
            }),
            _ => Game::Running(GameState {
                last_message: GameMessage {
                    contents: format!(
                        "Ye sepaketh nonsense, I know not the command '{}'. Try 'help'.",
                        inputs[0]
                    ),
                },
                ..self
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Game, GameState};
    use crate::inventory::ItemType;
    use crate::{Area, Inventory, Item};

    impl Game {
        fn assert_message(&self, expected: &str) {
            let expected = String::from(expected);
            match self {
                Game::Running(game_state) => {
                    assert_eq!(expected, game_state.last_message.contents);
                }
                Game::NotRunning(final_message) => {
                    assert_eq!(expected, final_message.to_owned());
                }
            }
        }

        fn assert_message_contains(&self, expected_pattern: &str) {
            match self {
                Game::Running(game_state) => {
                    assert_eq!(
                        true,
                        game_state.last_message.contents.contains(expected_pattern)
                    );
                }
                Game::NotRunning(final_message) => {
                    assert_eq!(true, final_message.to_owned().contains(expected_pattern));
                }
            }
        }

        fn process(self, command: &str) -> Game {
            match self {
                Game::Running(game_state) => game_state.process(String::from(command)),
                _ => panic!("Expected game to be running"),
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
        game.assert_message(
            "Your feet rest upon green meadows.\nYou look around, and see:\n1: Potion",
        );
    }

    #[test]
    fn respond_helpfully_to_unknown_commands() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("gibberish"));
        game.assert_message(
            "Ye sepaketh nonsense, I know not the command 'gibberish'. Try 'help'.",
        );
    }

    #[test]
    fn pickup_item() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("pickup 1"));
        game.assert_message("You pickup the Potion");
        let game = game.process("inventory");
        game.assert_message("Your inventory:\n1: Potion");
    }

    #[test]
    fn inventory_at_beginning() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("inventory"));
        game.assert_message("Your inventory:\nNothing.");
    }

    // #[test]
    // fn pickup_item_and_using_it_removes_the_item() {
    //     let game_state = GameState::new();
    //     let game = game_state.process(String::from("pickup 1"));
    //     let game = game.process("inventory");
    //     game.assert_message("Your inventory:\n1: Potion");
    //     let game = game.process("use 1");
    //     game.assert_message("Your inventory:\nNothing");
    // }

    #[test]
    fn picked_up_items_no_longer_in_area() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("pickup 1"));
        let game = game.process("look");
        game.assert_message(
            "Your feet rest upon green meadows.\nYou look around, and see:\nNothing.",
        );
    }

    #[test]
    fn print_help_message() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("help"));
        game.assert_message_contains("look");
        game.assert_message_contains("pickup");
        game.assert_message_contains("exit");
        game.assert_message_contains("help");
        game.assert_message_contains("inventory");
        game.assert_message_contains("status");
        game.assert_message_contains("use");
    }

    #[test]
    fn display_status() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("status"));
        game.assert_message("Level:1\nHealth:10");
    }

    #[test]
    fn drinking_potion_increases_health() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("pickup 1"));
        let game = game.process("use 1");
        game.assert_message("You drink the Potion.");
        let game = game.process("status");
        game.assert_message("Level:1\nHealth:11");
    }

    #[test]
    fn drinking_venom_decreases_health() {
        let game_state = GameState::from(Area::new(
            "Your feet rest upon green meadows.",
            vec![Item {
                name: String::from("Venom"),
                item_type: ItemType::Venom,
            }],
        ));
        let game = game_state.process(String::from("pickup 1"));
        let game = game.process("use 1");
        game.assert_message("You drink the venom (For some reason???).");
        let game = game.process("status");
        game.assert_message("Level:1\nHealth:9");
    }
}
