use std::io;

fn main() {
    println!("Paper, Scissors, Stone");
    println!("=====================");
    println!("1. Paper");
    println!("2. Scissors");
    println!("3. Stone");

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();

    let user_choice = user_choice.trim().parse::<i32>().unwrap();

    let computer_choice = (rand::random::<f64>() * 3.0).floor() as i32 + 1;

    println!(
        "You chose: {}",
        match user_choice {
            1 => "Paper",
            2 => "Scissors",
            3 => "Stone",
            _ => "Invalid choice",
        }
    );

    println!(
        "Computer chose: {}",
        match computer_choice {
            1 => "Paper",
            2 => "Scissors",
            3 => "Stone",
            _ => "Invalid choice",
        }
    );

    let result = match (user_choice, computer_choice) {
        (1, 2) => "You win!",
        (1, 3) => "You lose!",
        (2, 1) => "You lose!",
        (2, 3) => "You win!",
        (3, 1) => "You win!",
        (3, 2) => "You lose!",
        _ => "Tie",
    };

    println!("Result: {result}");
}
