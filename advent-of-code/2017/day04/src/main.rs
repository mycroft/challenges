use mkz_aoc::file;
use std::collections::HashSet;
use std::iter::FromIterator;

fn apply(lines: &Vec<String>, accept: impl Fn(String, String) -> bool) -> usize {
    lines
        .iter()
        .filter(|line| {
            0 == line
                .split_whitespace()
                .map(|x| line
                    .split_whitespace()
                    .filter(|y| accept(y.to_string(), x.to_string()))
                    .count())
                .filter(|x| *x != 1)
                .count()
        })
        .count()
}

fn solve1(lines: &Vec<String>) -> usize {
    apply(lines, |x, y| x == y)
}

fn solve2(lines: &Vec<String>) -> usize {
    let transform = |x: String| HashSet::from_iter(x.chars()) as HashSet<char>;

    apply(lines, |x, y| transform(x.to_string()) == transform(y.to_string()))
}

fn main() {
    let lines = file::read_to_lines("input.txt").unwrap();

    println!("Part #1: {:?}", solve1(&lines));
    println!("Part #2: {:?}", solve2(&lines));
}
