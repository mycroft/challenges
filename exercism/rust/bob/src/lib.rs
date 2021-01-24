pub fn reply(message: &str) -> &str {
    let message = message.trim_end();

    let is_question = message.ends_with("?");
    let is_all_upcase = message.chars().filter(|c| c.is_alphabetic()).all(char::is_uppercase);
    let has_letters = message.chars().any(char::is_alphabetic);

    if message.len() == 0 {
        "Fine. Be that way!"
    } else if is_question && has_letters && is_all_upcase {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if !has_letters {
        "Whatever."
    } else if !is_question && is_all_upcase {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
