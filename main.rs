use std::io::{self, BufRead}

enum State {
    Locked,
    UnLocked,
}
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
    for line in stdin.lock().lines(){
        println!("{}", line.unwrap())
    }
}
