/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 
        || !code.chars().all(|x| x == ' ' || x >= '0' && x <= '9') 
        || code.starts_with(' ')
    {
        return false;
    }

    let numbers = code
        .chars()
        .filter(|x| *x != ' ')
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    let lst = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let mut res = *x;

            if i % 2 == (numbers.len() % 2) {
                res = res * 2;
            }

            if res > 9 {
                res -= 9;
            }

            res
        })
        .collect::<Vec<u8>>();

    return lst.iter().sum::<u8>() % 10 == 0;
}
