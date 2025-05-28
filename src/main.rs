use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let guess_list = ["apple", "banana", "mango", "oranges"];

    loop {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..guess_list.len());
        let random_fruit = guess_list[index];

        println!("Guess the fruit!: {}", random_fruit);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();
                println!("Fruit selected is: {}", fruit_selected);

                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("Fruit not found in list: {}", fruit_selected);
                    continue;
                }

                if guess_checker(&fruit_selected, random_fruit) {
                    println!("🎉 Winner!! You guessed it right: {} 🎉", random_fruit);
                    println!("Restarting the game...\n");
                    continue; // RESTART instead of break
                } else {
                    println!("❌ Wrong guess. Try again!\n");
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        };
    }
}

fn guess_checker(user_input: &str, random_fruit: &str) -> bool {
    user_input == random_fruit
}
