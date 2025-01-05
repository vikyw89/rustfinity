#[derive(PartialEq)]
enum State {
    Start,
    A,
    B,
    C,
}

pub fn recognize_pattern(input: &str) -> bool {
    let mut state = State::Start;

    for c in input.chars() {
        state = match state {
            State::Start => match c {
                'a' => State::A,
                _ => return false,
            },
            State::A => match c {
                'b' => State::B,
                'c' => State::C,
                _ => return false,
            },
            State::B => match c {
                'b' => State::B,
                'c' => State::C,
                _ => return false,
            },
            State::C => match c {
                _ => return false,
            },
        };
    }

    let result = state == State::C;

    return result;
}
