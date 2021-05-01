/*
 * 2019-01
 */

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut sum : u32 = 0;
    let mut sum_rec : u32 = 0;

    for line in &lines {
        let mass : u32 = line.parse::<u32>().unwrap();

        sum += fuel(mass);
        sum_rec += fuel_rec(mass);
    }

    println!("Part #1: {}", sum);
    println!("Part #2: {}", sum_rec);
}

fn fuel(mass: u32) -> u32 {
    if mass < 6 {
        mass
    } else {
        (mass / 3) - 2
    }
}

fn fuel_rec(mass: u32) -> u32 {
    let mut total : u32 = 0;
    let mut mass = mass;

    while mass >= 6 {
        total += fuel(mass);
        mass = fuel(mass);
    }

    total
}

#[test]
fn test_fuel() {
    assert_eq!(2, fuel(12));
    assert_eq!(2, fuel(14));
    assert_eq!(654, fuel(1969));
    assert_eq!(33583, fuel(100756));
}

#[test]
fn test_fuel_rec() {
    assert_eq!(2, fuel_rec(14));
    assert_eq!(966, fuel_rec(1969));
    assert_eq!(50346, fuel_rec(100756));

}
