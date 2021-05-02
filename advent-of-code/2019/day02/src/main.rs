/*
 * 2019-02
 */
use std::fs;

fn run(contents: &String, noun: i32, verb: i32) -> i32 {
    let mut codes = contents
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut index = 0;

    codes[1] = noun;
    codes[2] = verb;

    while index < codes.len() {
        let opcode = *codes.get(index).unwrap();

        if opcode == 99 {
            break;
        }

        let r1 = *codes.get(index + 1).unwrap();
        let r2 = *codes.get(index + 2).unwrap();
        let r3 = *codes.get(index + 3).unwrap();

        if opcode == 1 {
            codes[r3 as usize] = *codes.get(r1 as usize).unwrap() + *codes.get(r2 as usize).unwrap();
        }

        if opcode == 2 {
            codes[r3 as usize] = *codes.get(r1 as usize).unwrap() * *codes.get(r2 as usize).unwrap();
        }


        index += 4;
    };

    *codes.get(0).unwrap()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    println!("Part #1: {}", run(&contents, 12, 2));

    let rech = 19690720;
    let mut result;

    let mut verb = 0;
    let mut noun = 0;

    loop {
        result = run(&contents, verb, noun);

        if result == rech {
            break;
        }

        if result > rech && noun == 0 {
            noun += 1;
            verb -= 1;
        }

        if noun == 0 {
            verb += 1;
        } else {
            noun += 1;
        }
    }

    println!("Part #2: {}", 100 * verb + noun);
}


#[test]
fn test_run() {
    assert_eq!(2, run(&"1,0,0,0,99".to_string(), 0, 0));
    assert_eq!(3500, run(&"1,9,10,3,2,3,11,0,99,30,40,50".to_string(), 9, 10));
    assert_eq!(2, run(&"2,3,0,3,99".to_string(), 3, 0));
    assert_eq!(2, run(&"2,4,4,5,99,0".to_string(), 4, 4));
    assert_eq!(30, run(&"1,1,1,4,99,5,6,0,99".to_string(), 1, 1));
}
