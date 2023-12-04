use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq)]
struct Piece {
    s: isize,
    value: u32,
}

fn check(symbols: &HashMap<Location, char>, pieces: &HashMap<Location, Piece>) -> u32 {
    let mut ret = 0;
    for (location, piece) in pieces {
        let mut has_symbol = false;

        for y in (location.y-1..location.y+2) {
            if y == -1 {
                continue;
            }

            for x in (location.x-1..location.x+piece.s+1) {
                if symbols.get(&Location { x: x, y: y }).is_some() {
                    has_symbol = true;
                }
            }
        }

        if has_symbol {
            // println!("Adding {}", piece.value);
            ret += piece.value;
        }
    }

    return ret;
}

fn check_gear_ratio(symbols: &HashMap<Location, char>, pieces: &HashMap<Location, Piece>) -> u32 {
    let mut ret = 0;

    for (location, symbol) in symbols {
        if *symbol != '*' {
            continue;
        }

        let mut pieces_total = 1;
        let mut pieces_num = 0;

        for (piece_location, piece) in pieces {
            for y in (piece_location.y-1..piece_location.y+2) {    
                for x in (piece_location.x-1..piece_location.x+piece.s+1) {
                    if (*location == Location{x: x, y: y}) {
                        pieces_num += 1;
                        pieces_total *= piece.value;
                    }
                }
            }    
        }

        if pieces_num == 2 {
            ret += pieces_total;
        }
    }

    return ret;
}

fn main() {
    let contents = read_to_string("input.txt").expect("file to open and read");
    let mut symbols : HashMap<Location, char> = HashMap::new();
    let mut pieces: HashMap<Location, Piece> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        let mut current_engine: Option<Piece> = None;
        let mut current_engine_idx: Option<isize> = None;
    
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if current_engine_idx.is_none() {
                    current_engine = Some(Piece{s: 1, value: c.to_digit(10).unwrap()});
                    current_engine_idx = Some(x as isize);
                } else {
                    let current_piece = current_engine.unwrap();
                    current_engine = Some(Piece { s: current_piece.s + 1, value: current_piece.value * 10 +  c.to_digit(10).unwrap()});
                }
                continue;
            } else {
                if current_engine != None {
                    pieces.insert(Location{x: current_engine_idx.unwrap(), y: y as isize}, current_engine.unwrap());
                    current_engine = None;
                    current_engine_idx = None;
                }
            }

            if c == '.' {
                continue;
            }

            // symbol
            symbols.insert(Location { x: x as isize, y: y as isize }, c);
        }

        if current_engine != None {
            pieces.insert(Location{x: current_engine_idx.unwrap(), y: y as isize}, current_engine.unwrap());
            current_engine = None;
            current_engine_idx = None;
        }
    }

    let p1 = check(&symbols, &pieces);
    let p2 = check_gear_ratio(&symbols, &pieces);

    println!("#1: {}", p1);
    println!("#2: {}", p2);
}
