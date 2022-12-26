use std::fs::read_to_string;

fn parse(fp: &str) -> Vec<i32> {
    let contents = read_to_string(fp).unwrap();
    
    contents.lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn compute(mass: i32) -> i32 {
    mass / 3 - 2
}

fn compute_rec(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;

    if fuel <= 0 {
        0
    } else {
        fuel + compute_rec(fuel)
    }
}

fn main() {
    println!("#1 {}", parse("input.txt").iter().map(|&x| compute(x)).sum::<i32>()); // 3184233
    println!("#1 {}", parse("input.txt").iter().map(|&x| compute_rec(x)).sum::<i32>()); // 4773483
}

#[test]
fn test_sample() {
    assert_eq!(2, compute(12));
    assert_eq!(2, compute(14));
    assert_eq!(654, compute(1969));
    assert_eq!(33583, compute(100756));
}

#[test]
fn test_sample2() {
    assert_eq!(2, compute_rec(12));
    assert_eq!(2, compute_rec(14));
    assert_eq!(966, compute_rec(1969));
    assert_eq!(50346, compute_rec(100756));
}