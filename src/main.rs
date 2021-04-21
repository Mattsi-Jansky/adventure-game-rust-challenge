use text_io::read;

fn main() {
    let mut game = Game::new();
    while game.is_running {
        println!("Please enter a command:");
        let line: String = read!("{}\n");
        game = game.process(line);
        println!("{}", game.last_message);
    }
}

struct Game {
    pub is_running: bool,
    pub last_message: String,
    player: Player,
    area: Area
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

    fn look(&self) -> String {
        let mut description = self.description.clone();
        description.push_str("\nYou look around, and see:\n");

        description.push_str(&self.inventory.iter().enumerate()
            .map(|(i, item)| format!("{}: {}", i + 1, item.name))
            .reduce(|a, b| format!("{}\n{}", a, b)).unwrap_or(String::from("Nothing."))[..]);

        description
    }
}

struct Item {
    name: String
}

struct Player {

}

impl Game {
    fn new() -> Game {
        Game {
            is_running: true,
            last_message: "".to_string(),
            player: Player {},
            area: Area::meadows()
        }
    }

    fn process(self, input: String) -> Game {
        let inputs = input.split_whitespace().collect::<Vec<&str>>();
        if inputs[0].eq("exit") {
            Game { last_message: String::from("Ye giveth up like a whiny little child"), is_running: false, ..self }
        }
        else if inputs[0].eq("look") {
            Game { last_message: self.area.look(), ..self }
        }
        else {
            Game { last_message: input, ..self }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Game;

    #[test]
    fn exit_changes_is_running_state_to_false() {
        let mut game = Game::new();
        assert_eq!(true, game.is_running);
        game = game.process("exit".to_string());
        assert_eq!(false, game.is_running);
    }

    #[test]
    fn look_around() {
        let mut game = Game::new();
        game = game.process("look".to_string());
        assert_eq!("Your feet rest upon green meadows.\nYou look around, and see:\n1: Potion", game.last_message)
    }
}
