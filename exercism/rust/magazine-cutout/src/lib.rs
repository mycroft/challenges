// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut parts : HashMap<&str, usize> = HashMap::new();
    let mut required_parts : HashMap<&str, usize> = HashMap::new();

    magazine.iter().map(|x| { *parts.entry(*x).or_insert(0) += 1}).count();
    note.iter().map(|x| { *required_parts.entry(*x).or_insert(0) += 1 }).count();

    required_parts.iter().all(|(k, v)| v <= parts.entry(k).or_insert(0))
}
