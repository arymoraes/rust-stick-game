extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    struct Game {
        total_sticks: u32,
        current_sticks: u32,
        round: u32,
        is_won: bool,
        last_stick_amount_taken: u32,
    }

    struct Player {
        name: String
    }

    fn create_game() -> Game {
        let sticks_number: u32 = rand::thread_rng().gen_range(10,30);
    
        return Game {
            total_sticks: sticks_number,
            current_sticks: sticks_number,
            round: 1,
            is_won: false,
            last_stick_amount_taken: 0,
        };
    }
    
    fn create_player() -> Player {
        println!("What is your name?");
        let mut player_name = String::new();
    
        io::stdin().read_line(&mut player_name)
            .expect("Failed to read your name.");
    
        println!("Player one name is: {}", player_name.trim());
        return Player {
            name: player_name,
        };
    }

    fn compute_round(current_player: &Player, mut game: Game) -> Game {
        let mut valid_round: bool = false;

        while !valid_round {
            println!("How many sticks are you getting, {}?", current_player.name.trim());

            let mut stick_amount = String::new();
            io::stdin().read_line(&mut stick_amount)
                .expect("Amount not valid");
    
            let input_number: u32 = stick_amount
                .trim()
                .parse()
                .expect("Wanted a number");

            if (input_number >= 1 && input_number <= 3 && input_number < game.current_sticks) {
                game.current_sticks = game.current_sticks - input_number;
                game.last_stick_amount_taken = input_number;
                valid_round = true;
            } else {
                println!("Please enter a valid amount, between 1-3 sticks.");
            }
        }

        return game;
    }

    let mut game = create_game();

    let player_one = create_player();
    let player_two = create_player();

    println!("Welcome to the Stick Game. The objective of this game is to force the other player to get the last stick. You can choose to remove 1, 2 or 3 sticks at a time. The first player to remove the last stick loses. Good luck!");

    println!("The number of sticks is: {}", game.total_sticks);

    ////////////////////////////////////////////////////

    println!("{} will go first.", player_one.name.trim());

    while !game.is_won {
        let mut current_player = &player_one;
        if game.round % 2 == 0 {
            current_player = &player_two;
        }

        game = compute_round(current_player, game);

        println!("{} took a total of {} sticks. We have {} sticks left on the stack.", current_player.name.trim(), game.last_stick_amount_taken.to_string().trim(), game.current_sticks);

        if game.current_sticks < 1 {
            game.is_won = true;
            println!("{} lost.", current_player.name.trim());
            return;
        } else if game.current_sticks == 1 {
            game.is_won = true;
            println!("{} won.", current_player.name.trim());
            return;
        }

        game.round = game.round + 1;
    }
}