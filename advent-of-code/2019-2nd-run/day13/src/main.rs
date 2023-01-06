use intcode::{parse,Machine};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl From<isize> for Tile {
    fn from(n: isize) -> Self {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => unreachable!()
        }
    }
}

// returns (x of paddle, x of ball, number of blocks, score)
fn output_to_state(output: &Vec<isize>) -> (isize, isize, isize, isize) {
    let mut idx = 0;
    let mut hm: HashMap<Position, Tile> = HashMap::new();
    let mut score = 0;

    while idx < output.len() {
        let x = output[idx];
        let y = output[idx+1];
        
        if x == -1 && y == 0 {
            score = output[idx+2];
            idx += 3;

            continue;
        }

        hm.insert(Position{x, y}, output[idx+2].into());
        idx += 3;
    }

    let number_of_blocks = hm.iter().filter(|(_, &x)| x == Tile::Block).count();
    let paddle_x = hm.iter().filter(|(_, &x)| x == Tile::HorizontalPaddle).nth(0).unwrap().0.x;
    let ball_x = hm.iter().filter(|(_, &x)| x == Tile::Ball).nth(0).unwrap().0.x;

    (paddle_x, ball_x, number_of_blocks as isize, score)
}

fn main() {
    let mut code = parse("input.txt");
    let mut machine = Machine::new(&code);

    machine.run();
    let output = machine.get_output();

    let (_, _, blocks, _) = output_to_state(&output);
    println!("#1 {blocks}");

    // step2
    code[0] = 2;
    let mut machine = Machine::new(&code);

    loop {
        machine.run();
        let (paddle_x, ball_x, blocks_remaining, score) = output_to_state(&machine.get_output());

        let input = if paddle_x > ball_x {
            -1
        } else if paddle_x < ball_x {
            1
        } else {
            0
        };

        machine.add_input(input);

        if blocks_remaining == 0 {
            println!("#2 {score}");
            break;
        }

        // println!("paddle:{paddle_x} ball:{ball_x} blocks: {blocks_remaining} score: {score}");
    }
}
