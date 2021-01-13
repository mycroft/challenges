use std::fs;

fn count(from: &Vec<bool>) -> usize {
    from.iter().filter(|x| !**x).count()
}

fn step(from: &Vec<bool>) -> (Vec<bool>, usize) {
    let mut out : Vec<bool> = vec![false; from.len()];
    let mut cnt = 0;

    for (i, _b) in from.iter().enumerate() {
        let mut left = false;
        let mut right = false;

        if i > 0 {
            left = from[i - 1];
        }
        if i < from.len() - 1 {
            right = from[i + 1];
        }

        out[i] = (left && !right) || (!left && right);
        if !out[i] {
            cnt += 1;
        }
    }

    (out, cnt)
}

fn get_input(filename: &str) -> Vec<bool> {
    let input = fs::read_to_string(filename).unwrap();

    input.trim_end().chars().map(|x| x == '^').collect::<Vec<bool>>()
}

#[allow(dead_code)]
fn dump(rows: &Vec<Vec<bool>>) {
    for i in 0..rows.len() {
        println!("{}: {}", i, rows[i].iter().map(|x| if *x { '^' } else { '.' }).collect::<String>());
    }
}

fn main() {
    let mut row = get_input("input.txt");
    let mut total_count = count(&row);

    for _i in 1..40 {
        let res = step(&row);

        total_count += res.1;
        row = res.0;
    }

    println!("Part #1: {}", total_count);

    let mut row = get_input("input.txt");
    let mut total_count = count(&row);

    for _i in 1..400000 {
        let res = step(&row);

        total_count += res.1;
        row = res.0;
    }

    println!("Part #2: {}", total_count);

}

#[test]
fn example() {
    assert_eq!((vec![false, true, true, true, true], 1), step(&vec![false, false, true, true, false]));
    assert_eq!((vec![true, true, false, false, true], 2), step(&vec![false, true, true, true, true]));
}