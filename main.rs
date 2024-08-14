use std::io::{self, BufRead, Write};
#[derive(Debug)]
enum State {
    Locked,
    UnLocked,
}
#[derive(Debug)]
enum Event {
    Push,
    Coin,
}

fn next_state(state: State, event: Event) -> State {
    match state {
        State::Locked => match event {
            Event::Push => State::Locked,
            Event::Coin => State::UnLocked,
        },
        State::UnLocked => match event {
            Event::Push => State::Locked,
            Event::Coin => State::UnLocked,
        },
    }
}

fn main() {
    //default state

    let mut state = State::Locked;

    let stdin = io::stdin();
    println!("State: {:?}", state);
    print!(">");
    for line in stdin.lock().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, Event::Coin),
            "push" => state = next_state(state, Event::Push),
            unknown => {
                eprintln!("Error:Unknown Event {}", unknown);
            }
        }
        println!("State: {:?}", state);
        print!(">");
        io::stdout().flush().unwrap();
    }
}
