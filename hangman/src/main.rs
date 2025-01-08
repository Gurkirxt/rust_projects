use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use rand::seq::SliceRandom;
use std::fs;
use std::io;

fn hangman() -> i8 {
    let mut t = 7;
    let filestr = fs::read_to_string("./words.txt").unwrap();
    let words: Vec<String> = filestr
        .to_lowercase()
        .split_whitespace()
        .map(String::from)
        .collect();
    let hangman_art = [
        r#"
  +---+
  |   |
      |
      |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
      |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,
    ];
    let mut rng = rand::thread_rng();
    let random_word = words.choose(&mut rng).unwrap().to_string();
    let mut input_list: Vec<char> = vec![];
    let mut guess: String = String::new();
    for ch in random_word.chars() {
        if input_list.contains(&ch) {
            guess.push(ch);
        } else {
            guess.push('_');
        }
    }
    while t > 0 {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        execute!(io::stdout(), cursor::MoveTo(1, 1)).unwrap();
        println!("{}", hangman_art[7 - t]);
        for c in guess.chars() {
            print!("{} ", c);
        }
        println!();
        println!("Enter your Guess!!");
        let mut input = String::new();
        let mut input_char: char = 'a';
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input_char = input.trim().to_string().chars().nth(0).unwrap();
                if input_list.contains(&input_char) {
                    println!("Character Already Guessed!!!\nGuess Again");
                    continue;
                } else {
                    input_list.push(input_char);
                }
            }
            Err(error) => println!("Error : {}", error),
        }
        if !random_word.contains(input_char) {
            t -= 1;
        }
        guess.clear();

        for ch in random_word.chars() {
            if input_list.contains(&ch) {
                guess.push(ch);
            } else {
                guess.push('_');
            }
        }
        if guess.contains('_') {
            continue;
        } else {
            execute!(io::stdout(), Clear(ClearType::All)).unwrap();
            execute!(io::stdout(), cursor::MoveTo(1, 1)).unwrap();
            println!("{}", hangman_art[7 - t]);
            for c in guess.chars() {
                print!("{} ", c);
            }
            return 1;
        }
    }
    println!("The Word was {}", random_word);
    return 0;
}

fn main() {
    let mut outcome: i8 = -1;
    let outcome_text = [
        r#" 
▗▖  ▗▖▗▄▖ ▗▖ ▗▖    ▗▖ ▗▖ ▗▄▖ ▗▖  ▗▖
 ▝▚▞▘▐▌ ▐▌▐▌ ▐▌    ▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌
  ▐▌ ▐▌ ▐▌▐▌ ▐▌    ▐▌ ▐▌▐▌ ▐▌▐▌ ▝▜▌
  ▐▌ ▝▚▄▞▘▝▚▄▞▘    ▐▙█▟▌▝▚▄▞▘▐▌  ▐▌
"#,
        r#"
▗▖  ▗▖▗▄▖ ▗▖ ▗▖    ▗▖    ▗▄▖  ▗▄▄▖▗▄▄▄▖
 ▝▚▞▘▐▌ ▐▌▐▌ ▐▌    ▐▌   ▐▌ ▐▌▐▌     █  
  ▐▌ ▐▌ ▐▌▐▌ ▐▌    ▐▌   ▐▌ ▐▌ ▝▀▚▖  █  
  ▐▌ ▝▚▄▞▘▝▚▄▞▘    ▐▙▄▄▖▝▚▄▞▘▗▄▄▞▘  █  
"#,
    ];
    loop {
        if outcome == -1 {
            execute!(io::stdout(), Clear(ClearType::All)).unwrap();
            execute!(io::stdout(), cursor::MoveTo(1, 1)).unwrap();
            let hangman_ascii = r#"
██╗  ██╗ █████╗ ███╗   ██╗ ██████╗ ███╗   ███╗ █████╗ ███╗   ██╗     ██████╗  █████╗ ███╗   ███╗███████╗
██║  ██║██╔══██╗████╗  ██║██╔════╝ ████╗ ████║██╔══██╗████╗  ██║    ██╔════╝ ██╔══██╗████╗ ████║██╔════╝
███████║███████║██╔██╗ ██║██║  ███╗██╔████╔██║███████║██╔██╗ ██║    ██║  ███╗███████║██╔████╔██║█████╗  
██╔══██║██╔══██║██║╚██╗██║██║   ██║██║╚██╔╝██║██╔══██║██║╚██╗██║    ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  
██║  ██║██║  ██║██║ ╚████║╚██████╔╝██║ ╚═╝ ██║██║  ██║██║ ╚████║    ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗
╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝     ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝
            "#;
            println!("{}", hangman_ascii);
            print!("Enter S to Start\nEnter Q to Quit\n");
        } else if outcome == 1 {
            print!("{}", outcome_text[0]);
            print!("Enter S to Start Again\nEnter Q to Quit\n");
        } else if outcome == 0 {
            print!("{}", outcome_text[1]);
            print!("Enter S to Start Again\nEnter Q to Quit\n");
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
            }
            Err(error) => println!("Error : {}", error),
        }
        if input == "S" || input == "s" {
            outcome = hangman();
        } else {
            break;
        }
    }
}
