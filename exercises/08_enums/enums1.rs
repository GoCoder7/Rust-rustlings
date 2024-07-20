#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit
}

fn main() {
    let message = Message::Resize;
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
