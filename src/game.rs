use crate::inventory::Inventory;
use crate::area::Area;

pub struct GameMessage {
    contents: String
}

impl GameMessage {
    pub(crate) fn new(contents: &str) -> GameMessage {
        GameMessage { contents: String::from(contents) }
    }

    pub(crate) fn print(&self) {
        println!("{}", self.contents);
    }
}

pub enum Game {
    Running(GameState),
    NotRunning(String)
}

pub struct GameState {
    pub last_message: GameMessage,
    inventory: Inventory,
    area: Area
}

impl GameState {
    pub(crate) fn new() -> GameState {
        GameState {
            last_message: GameMessage { contents: String::new() },
            inventory: Inventory::new(),
            area: Area::meadows()
        }
    }

    pub(crate) fn process(self, input: String) -> Game {
        let inputs = input.split_whitespace().collect::<Vec<&str>>();
        match inputs[0] {
            "look" => {
                Game::Running(GameState { last_message: self.area.look(), ..self })
            }
            "pickup" => {
                let index = inputs[1].parse::<usize>().unwrap() - 1;
                let item = self.area.get_from_inventory(&index);
                Game::Running(GameState {
                    last_message: GameMessage { contents: String::from(format!("You pickup the {}", item.name)) },
                    inventory: self.inventory.with(item),
                    area: self.area.without_item(index),
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
    use crate::game::{Game, GameState};

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
    fn print_help_message() {
        let game_state = GameState::new();
        let game = game_state.process(String::from("help"));
        game.assert_message("Known commands are:\nlook\npickup\nexit\nhelp");
    }
}
