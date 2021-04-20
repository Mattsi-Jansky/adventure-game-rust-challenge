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
    description: String
}

struct Player {

}

impl Game {
    fn new() -> Game {
        Game {
            is_running: true,
            last_message: "".to_string(),
            player: Player {},
            area: Area { description: String::from("Your feet rest upon green meadows.") }
        }
    }

    fn process(self, input: String) -> Game {
        let inputs = input.split_whitespace().collect::<Vec<&str>>();;
        if inputs[0].eq("exit") {
            Game { last_message: String::from("Ye giveth up like a whiny little child"), is_running: false, ..self }
        }
        else if inputs[0].eq("look") {
            Game { last_message: self.area.description.clone(), ..self }
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
        assert_eq!("Your feet rest upon green meadows.", game.last_message)
    }
}
