use std::collections::HashMap;

#[derive(PartialEq,Clone,Copy)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

struct Action {
    InitialState: State,
    NextState: State,
    InitialValue: usize,
    NextValue: usize,
    NextDir: Direction
}

fn find_action(actions: &Vec<Action>, initialState: State, InitialValue: usize) -> Option<&Action> {
    for action in actions {
        if initialState == action.InitialState && InitialValue == action.InitialValue {
            return Some(action);
        }
    }

    None
}

fn main() {
    let actions = vec![
        Action{InitialState: State::A, InitialValue: 0, NextValue: 1, NextDir: Direction::Right, NextState: State::B},
        Action{InitialState: State::A, InitialValue: 1, NextValue: 0, NextDir: Direction::Left, NextState: State::C},

        Action{InitialState: State::B, InitialValue: 0, NextValue: 1, NextDir: Direction::Left, NextState: State::A},
        Action{InitialState: State::B, InitialValue: 1, NextValue: 1, NextDir: Direction::Left, NextState: State::D},

        Action{InitialState: State::C, InitialValue: 0, NextValue: 1, NextDir: Direction::Right, NextState: State::D},
        Action{InitialState: State::C, InitialValue: 1, NextValue: 0, NextDir: Direction::Right, NextState: State::C},

        Action{InitialState: State::D, InitialValue: 0, NextValue: 0, NextDir: Direction::Left, NextState: State::B},
        Action{InitialState: State::D, InitialValue: 1, NextValue: 0, NextDir: Direction::Right, NextState: State::E},

        Action{InitialState: State::E, InitialValue: 0, NextValue: 1, NextDir: Direction::Right, NextState: State::C},
        Action{InitialState: State::E, InitialValue: 1, NextValue: 1, NextDir: Direction::Left, NextState: State::F},

        Action{InitialState: State::F, InitialValue: 0, NextValue: 1, NextDir: Direction::Left, NextState: State::E},
        Action{InitialState: State::F, InitialValue: 1, NextValue: 1, NextDir: Direction::Right, NextState: State::A},
    ];

    let mut current_position = 0;
    let mut current_state = State::A;

    let mut states = HashMap::new();

    for index in 0..12656374 {
        let mut current_value = states.entry(current_position).or_insert(0);
        let mut current_action = find_action(&actions, current_state, *current_value).unwrap();

        *current_value = current_action.NextValue;

        if current_action.NextDir == Direction::Left {
            current_position -= 1;
        } else {
            current_position += 1;
        }

        current_state = current_action.NextState;
    }

    println!("{:?}", states.iter().map(|(k, x)| x).sum::<usize>());
}
