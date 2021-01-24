use std::fs;
use regex::Regex;
use std::collections::HashMap;

enum Type {
    Rotate,
    Swap,
    Swapletter
}

struct Inst {
    t: Type,
    a1: u8,
    a2: u8,
}

fn main() {
    let _contents = fs::read_to_string("input.txt").unwrap();
    let _contents = _contents.trim_end();
    let _parts = _contents.split(",");

    let r_s = Regex::new(r"^s(\d+)$").unwrap();
    let r_x = Regex::new(r"^x(\d+)/(\d+)$").unwrap();
    let r_p = Regex::new(r"^p(.)/(.)$").unwrap();

    let mut buffer = "abcdefghijklmnop".chars().collect::<Vec<char>>();

    let mut insts = vec![];

    for p in _parts {
        if r_s.is_match(p) {
            let caps = r_s.captures(p).unwrap();
            let rotate = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

            insts.push(Inst{ t: Type::Rotate, a1: rotate as u8, a2: 0 });

            for _i in 0..rotate {
                let v = buffer.pop().unwrap();
                buffer.insert(0, v);
            }
        } else if r_x.is_match(p) {
            let caps = r_x.captures(p).unwrap();

            let r0 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let r1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

            insts.push(Inst{ t: Type::Swap, a1: r0 as u8, a2: r1 as u8 });

            let c = buffer[r0];
            buffer[r0] = buffer[r1];
            buffer[r1] = c;
        } if r_p.is_match(p) {
            let caps = r_p.captures(p).unwrap();

            let c1 = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
            let c2 = caps.get(2).unwrap().as_str().chars().nth(0).unwrap();

            let pos1 : usize = buffer
                .iter()
                .enumerate()
                .filter(|(_p, &x)| x == c1)
                .map(|(p, _x)| p)
                .nth(0)
                .unwrap();

            let pos2 : usize = buffer
                .iter()
                .enumerate()
                .filter(|(_p, &x)| x == c2)
                .map(|(p, _x)| p)
                .nth(0)
                .unwrap();

            insts.push(Inst{ t: Type::Swapletter, a1: c1 as u8, a2: c2 as u8 });

            let c = buffer[pos1];
            buffer[pos1] = buffer[pos2];
            buffer[pos2] = c;
        }
    }

    println!("Part #1: {}", buffer.iter().collect::<String>());



    let mut buffer = "abcdefghijklmnop".chars().collect::<Vec<char>>();

    let mut h = HashMap::new();
    let mut idx = 0;

    let limit = 1_000_000_000;

    loop {
        for inst in &insts {
            match inst.t {
                Type::Rotate => {
                    for _i in 0..inst.a1 {
                        let v = buffer.pop().unwrap();
                        buffer.insert(0, v);
                    }
                },
                Type::Swap => {
                    let c = buffer[inst.a1 as usize];
                    buffer[inst.a1 as usize] = buffer[inst.a2 as usize];
                    buffer[inst.a2 as usize] = c;
                },
                Type::Swapletter => {
                    let pos1 : usize = buffer
                        .iter()
                        .enumerate()
                        .filter(|(_p, &x)| x == (inst.a1 as char))
                        .map(|(p, _x)| p)
                        .nth(0)
                        .unwrap();

                    let pos2 : usize = buffer
                        .iter()
                        .enumerate()
                        .filter(|(_p, &x)| x == (inst.a2 as char))
                        .map(|(p, _x)| p)
                        .nth(0)
                        .unwrap();

                    let c = buffer[pos1];
                    buffer[pos1] = buffer[pos2];
                    buffer[pos2] = c;
                }
            }
        }

        idx += 1;

        let s = buffer.iter().collect::<String>();

        if let Some(v) = h.get(&s) {
            idx = limit - (limit % (idx - v)) + 1;

            h.clear();            
        }

        h.insert(s, idx);

        if idx >= limit {
            break;
        }
    }

    println!("Part #2: {}", buffer.iter().collect::<String>());
}
