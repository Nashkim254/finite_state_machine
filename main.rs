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

fn main() {}
