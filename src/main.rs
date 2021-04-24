use text_io::read;

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
    inventory: Vec<Item>
}

impl Area {
    fn meadows() -> Area {
        Area {
            description: String::from("Your feet rest upon green meadows."),
            inventory: vec![ Item { name: String::from("Potion") } ]
        }
    }

    fn look(&self) -> GameMessage {
        let mut description = self.description.clone();
        description.push_str("\nYou look around, and see:\n");

        description.push_str(&self.inventory.iter().enumerate()
            .map(|(i, item)| format!("{}: {}", i + 1, item.name))
            .reduce(|a, b| format!("{}\n{}", a, b)).unwrap_or(String::from("Nothing."))[..]);

        GameMessage { contents: description }
    }
}

struct Item {
    name: String
}

struct Player {

}

enum Game {
    Running(GameState),
    NotRunning(String)
}

struct GameState {
    pub last_message: GameMessage,
    player: Player,
    area: Area
}

impl GameState {
    fn new() -> GameState {
        GameState {
            last_message: GameMessage { contents: String::new() },
            player: Player {},
            area: Area::meadows()
        }
    }

    fn process(self, input: String) -> Game {
        let inputs = input.split_whitespace().collect::<Vec<&str>>();
        if inputs[0].eq("exit") {
            Game::NotRunning(String::from("Ye hath not the faith to go on"))
        }
        else if inputs[0].eq("look") {
            Game::Running(GameState { last_message: self.area.look(), ..self })
        }
        else {
            Game::Running(GameState { last_message: GameMessage { contents: input }, ..self })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GameState, Game};

    impl Game {
        fn assert_message(self, expected: String) {
            match self {
                Game::Running(game_state) => {
                    assert_eq!(expected, game_state.last_message.contents);
                },
                Game::NotRunning(final_message) => {
                    assert_eq!(expected, final_message);
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
        game.assert_message(String::from("Your feet rest upon green meadows.\nYou look around, and see:\n1: Potion"));
    }
}
