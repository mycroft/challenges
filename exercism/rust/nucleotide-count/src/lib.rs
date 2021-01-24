use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let cnts = nucleotide_counts(dna)?;

    match cnts.get(&nucleotide) {
        Some(v) => Ok(*v),
        None => Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let valid = ['A', 'G', 'T', 'C'];
    let mut cnt : HashMap<char, usize> = valid.iter().map(|x| (*x, 0)).collect();

    for c in dna.chars() {
        match cnt.get_mut(&c) {
            Some(n) => *n += 1,
            None => return Err(c),
        }
    }

    Ok(cnt)
}
