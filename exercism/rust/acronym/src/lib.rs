pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c| c == ' ' || c == '-' || c == '_')
        .filter(|x| *x != "")
        .map(|word| word
            .chars()
            .enumerate()
            .filter(move |(i, x)| *i == 0 || (x.is_uppercase() && !word.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase)))
            .map(|(_i, x)| x.to_ascii_uppercase())
        )
        .flatten()
        .collect()
}
