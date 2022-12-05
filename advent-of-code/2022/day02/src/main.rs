use core::panic;
use std::fs;

enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Type {
    Win,
    Draw,
    Loss,
}

fn letter_to_play(x: &str) -> Play {
    match x {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => { panic!("could not convert letter") }
    }
}

fn letter_to_win(x: &str) -> Type {
    match x {
        "X" => Type::Loss,
        "Y" => Type::Draw,
        "Z" => Type::Win,
        _ => { panic!("could not convert letter") }
    }
}

fn score(a: Play, b: Play) -> i32 {
    let score = match a {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };

    let playtype = match a {
        Play::Rock => {
            match b {
                Play::Paper => Type::Loss,
                Play::Scissors => Type::Win,
                Play::Rock => Type::Draw,
            }
        },
        Play::Paper => {
            match b {
                Play::Paper => Type::Draw,
                Play::Scissors => Type::Loss,
                Play::Rock => Type::Win,
            }
        },
        Play::Scissors => {
            match b {
                Play::Paper => Type::Win,
                Play::Scissors => Type::Draw,
                Play::Rock => Type::Loss,
            }
        },
    };

    return score + match playtype {
        Type::Win => 6,
        Type::Loss => 0,
        Type::Draw => 3,
    }
}

fn expected_play(a: Play, b: Type) -> Play {
    match b {
        Type::Win => match a {
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
            Play::Rock => Play::Paper,
        },
        Type::Draw => a,
        Type::Loss => match a {
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
            Play::Rock => Play::Scissors,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("need a file");
    let lines : Vec<&str> = contents.split("\n").collect();
    let games = lines.iter().map(|x| x.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let mut final_score = 0;
    let mut final_score_step2 = 0;

    for game in games {
        if game.len() != 2 {
            break;
        }
        final_score += score(
            letter_to_play(game[1]),
            letter_to_play(game[0])
        );

        final_score_step2 += score(
            expected_play(
                letter_to_play(game[0]),
                letter_to_win(game[1]),
            ),
            letter_to_play(game[0]),
        );
    }

    println!("#1: {}", final_score);
    println!("#2: {}", final_score_step2);
}
