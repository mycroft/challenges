use std::collections::HashSet;
use std::collections::HashMap;

pub fn get_map(word: &str) -> HashMap<char,usize> {
    let mut hm : HashMap<char,usize> = HashMap::new();

    word
        .chars()
        .map(|c| c.to_lowercase().next().unwrap())
        .map(|c| *hm.entry(c).or_insert(0) += 1)
        .count();

    hm
}

pub fn is_anagram(word1: &str, word2: &str) -> bool {
    get_map(word1) == get_map(word2) && word1.to_lowercase() != word2.to_lowercase()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res : HashSet<&'a str> = HashSet::new();

    possible_anagrams
        .iter()
        .filter(|w| is_anagram(w, word))
        .map(|x| res.insert(x.clone()))
        .count();

    res
}
