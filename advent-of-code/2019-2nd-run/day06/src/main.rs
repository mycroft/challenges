use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

fn parse(fp: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let contents = read_to_string(fp).unwrap();

    for line in contents.lines() {
        let parts = line.split(")").collect::<Vec<&str>>();

        result.insert(
            parts[1].to_string(),
            parts[0].to_string(),
        );
    }

    result
}

fn how_many_orbits(arcs: &HashMap<String, String>, star: String) -> usize {
    let mut result = 0;
    let mut current_star = star;

    while let Some(star) = arcs.get(&current_star) {
        result += 1;
        current_star = star.to_string();
    }

    result
}

fn get_orbits(arcs: &HashMap<String, String>, star: String) -> HashSet<String> {
    let mut result = HashSet::new();
    let mut current_star = star;

    while let Some(star) = arcs.get(&current_star) {
        result.insert(star.to_string());
        current_star = star.to_string();
    }

    result
}

fn path_size(arcs: &HashMap<String, String>, star0: String, star1: String) -> usize {
    let hs0 = get_orbits(arcs, star0);
    let hs1 = get_orbits(arcs, star1);

    let inter = hs0.intersection(&hs1);
    let union = hs0.union(&hs1);

    union.count() - inter.count()
}

fn how_many_orbits_total(arcs: &HashMap<String, String>) -> usize {
    let mut result = 0;

    for k in arcs {
        result += how_many_orbits(arcs, k.0.to_string());
    }

    result
}

fn main() {
    let arcs = parse("input.txt");
    println!("#1 {}", how_many_orbits_total(&arcs));
    println!("#2 {}", path_size(&arcs, "YOU".to_string(), "SAN".to_string()));
}

#[test]
fn test_sample_test0() {
    let arcs = parse("input.txt_test0");

    assert_eq!(
        0,
        how_many_orbits(&arcs, "COM".to_string())
    );

    assert_eq!(
        3,
        how_many_orbits(&arcs, "D".to_string())
    );

    assert_eq!(
        7,
        how_many_orbits(&arcs, "L".to_string())
    );

    assert_eq!(
        42,
        how_many_orbits_total(&arcs)
    );
}

#[test]
fn test_sample_test1() {
    let arcs = parse("input.txt_test1");

    assert_eq!(
        HashSet::<String>::from_iter(["K", "J", "E", "D", "C", "B", "COM"].to_vec().iter().map(|x| x.to_string())),
        get_orbits(&arcs, "YOU".to_string())
    );

    assert_eq!(
        HashSet::<String>::from_iter(["I", "D", "C", "B", "COM"].to_vec().iter().map(|x| x.to_string())),
        get_orbits(&arcs, "SAN".to_string())
    );

    assert_eq!(
        4,
        path_size(&arcs, "YOU".to_string(), "SAN".to_string())
    );
}