use std::fs;
use regex::Regex;

fn swap(input: &String, pos1: usize, pos2: usize) -> String {
    let mut arr = input.as_bytes().to_vec();

    let tmp = arr[pos2];
    arr[pos2] = arr[pos1];
    arr[pos1] = tmp;

    String::from_utf8(arr.to_vec()).unwrap()
}

fn swap_letters(input: &String, letter1: char, letter2: char) -> String {
    let pos1 = input.find(|c| c == letter1).unwrap();
    let pos2 = input.find(|c| c == letter2).unwrap();

    swap(input, pos1, pos2)
}

fn reverse(input: &String, from: usize, to: usize) -> String {
    let mut out = String::from(&input[..from]);

    out.push_str(&input[from..to+1].chars().rev().collect::<String>());
    out.push_str(&input[to+1..]);


    out
}

fn rotate_left(input: &String, iterations: usize) -> String {
    let mut out = input.to_string();
    let mut done = 0;

    while done != iterations {
        let letter = out.chars().nth(0).unwrap();
        out = String::from(&out[1..]);
        out.push(letter);

        done += 1;
    }


    out
}

fn rotate_right(input: &String, iterations: usize) -> String {
    let mut out = input.to_string();
    let mut done = 0;

    while done != iterations {
        let letter = out.chars().nth(out.len()-1).unwrap();
        let mut out2 = String::from(letter);
        out2.push_str(&String::from(&out[0..out.len()-1]));

        out = out2;
        done += 1;
    }

    out
}

fn move_char(input: &String, from: usize, to: usize) -> String {
    let mut out = input.to_string();
    let letter = out.remove(from);
    out.insert(to, letter);

    out
}

fn rotate_letter(input: &String, letter: char) -> String {
    let mut idx = input.find(|c| c == letter).unwrap();

    if idx >= 4 {
        idx += 1;
    }

    rotate_right(&input, idx + 1)
}

fn exec_line(
    line: &String,
    re_move: &Regex,
    re_swap: &Regex,
    re_reverse: &Regex,
    re_rotate: &Regex,
    re_rotate_letter: &Regex,
    re_swap_letter: &Regex,
    input: String,
    reverse_petterns: bool) -> String {
    
    if re_move.is_match(line) {
        let captures = re_move.captures(line).unwrap();

        if !reverse_petterns {
            move_char(&input,
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
            )
        } else {
            move_char(&input,
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap()
            )
        }

    } else if re_swap.is_match(line) {
        let captures = re_swap.captures(line).unwrap();

        // unchanged
        swap(&input,
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
        )
    } else if re_reverse.is_match(line) {
        let captures = re_reverse.captures(line).unwrap();

        // unchanged
        reverse(&input,
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
        )
    } else if re_rotate.is_match(line) {
        let captures = re_rotate.captures(line).unwrap();
        let iterations = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        if !reverse_petterns {
            match captures.get(1).unwrap().as_str() {
                "left" => rotate_left(&input, iterations),
                "right" => rotate_right(&input, iterations),
                _ => unreachable!()
            }
        } else {
            match captures.get(1).unwrap().as_str() {
                "right" => rotate_left(&input, iterations),
                "left" => rotate_right(&input, iterations),
                _ => unreachable!()
            }
        }
    } else if re_rotate_letter.is_match(line) {
        let captures = re_rotate_letter.captures(line).unwrap();

        let letter = captures.get(1).unwrap().as_str().chars().nth(0).unwrap();

        if !reverse_petterns {
            rotate_letter(&input, letter)
        } else {
            let mut test = input.to_string();

            while input != rotate_letter(&test, letter) {
                test = rotate_right(&test, 1);
            }

            test
        }
    } else if re_swap_letter.is_match(line) {
        let captures = re_swap_letter.captures(line).unwrap();

        // unchanged
        swap_letters(&input,
            captures.get(1).unwrap().as_str().chars().nth(0).unwrap(),
            captures.get(2).unwrap().as_str().chars().nth(0).unwrap(),
        )
    } else {
        String::from("")
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = contents.lines().collect::<Vec<&str>>();

    let re_move = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
    let re_swap = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let re_reverse = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let re_rotate = Regex::new(r"rotate (.*) (\d+) steps?").unwrap();
    let re_rotate_letter = Regex::new(r"rotate based on position of letter (.)").unwrap();
    let re_swap_letter = Regex::new(r"swap letter (.) with letter (.)").unwrap();

    let mut input = String::from("abcdefgh");
    
    for line in &lines {
        input = exec_line(
            &line.to_string(), 
            &re_move, &re_swap, &re_reverse,
            &re_rotate, &re_rotate_letter, &re_swap_letter,
            input,
            false
        );
    }

    println!("Part #1: {}", input);

    let mut input = String::from("fbgdceah");

    lines.reverse();

    for line in &lines {
        input = exec_line(
            &line.to_string(), 
            &re_move, &re_swap, &re_reverse,
            &re_rotate, &re_rotate_letter, &re_swap_letter,
            input,
            true
        );
    }

    println!("Part #2: {}", input);
}

#[test]
fn test() {
    let input = String::from("abcde");

    let res_a = swap(&input, 4, 0);
    assert_eq!(String::from("ebcda"), res_a);

    let res_b = swap_letters(&res_a, 'd', 'b');
    assert_eq!(String::from("edcba"), res_b);

    let res_c = reverse(&res_b, 0, 4);
    assert_eq!(String::from("abcde"), res_c);

    let res_d = rotate_left(&res_c, 1);
    assert_eq!(String::from("bcdea"), res_d);

    let res_tmp = rotate_right(&res_c, 1);
    assert_eq!(String::from("eabcd"), res_tmp);

    let res_e = move_char(&res_d, 1, 4);
    assert_eq!(String::from("bdeac"), res_e);

    let res_f = move_char(&res_e, 3, 0);
    assert_eq!(String::from("abdec"), res_f);

    let res_g = rotate_letter(&res_f, 'b');
    assert_eq!(String::from("ecabd"), res_g);

    let res_h = rotate_letter(&res_g, 'd');
    assert_eq!(String::from("decab"), res_h);

}

/*



rotate based on position of letter b finds the index of letter b (1),
then rotates the string right once plus a number of times equal to that index (2): ecabd.

rotate based on position of letter d
finds the index of letter d (4), 
then rotates the string right once, plus a number of times equal to that index, 
plus an additional time because the index was at least 4, for a total of 6 right rotations: decab.
*/
