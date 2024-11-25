use std::{thread, time, io};
use std::io::Write;

fn main() {
    println!("Welcome to the CLI Pomodoro Timer!");
    println!("Enter pomodoro duration (in minutes): ");
    let work_duration = read_minutes();

    println!("Enter break duration (in minutes): ");
    let break_duration = read_minutes();

    println!("Enter the number of Pomodoro sessions: ");
    let sessions = read_sessions();

    for session in 1..=sessions {
        println!("\nPomodoro Session {}: Work Time! 🚀", session);
        run_timer(work_duration);

        if session < sessions {
            println!("Take a Break! ☕");
            run_timer(break_duration);
        } else {
            println!("All sessions completed! 🎉");
        }
    }
}

fn read_minutes() -> u64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<u64>() {
            Ok(minutes) if minutes > 0 => return minutes,
            _ => println!("Please enter a valid positive number."),
        }
    }
}

fn read_sessions() -> u64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<u64>() {
            Ok(sessions) if sessions > 0 => return sessions,
            _ => println!("Please enter a valid positive number."),
        }
    }
}

fn run_timer(minutes: u64) {
    let total_seconds = minutes * 60;
    for remaining in (1..=total_seconds).rev() {
        print!("\rTime remaining: {:02}:{:02}", remaining / 60, remaining % 60);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
    println!("\rTime's up!                        ");
}

