enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to rgb({}, {}, {})", r, g, b),
    }
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write("Hi".to_string());
    let msg3 = Message::ChangeColor(255,255,255);
    let msg4 = Message::Quit;
    process_message(msg);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}

