fn get(n : u32) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20..=99 => {
            let base = match n {
                20..=29 => { "twenty" },
                30..=39 => { "thirty" },
                40..=49 => { "forty" },
                50..=59 => { "fifty" },
                60..=69 => { "sixty" },
                70..=79 => { "seventy" },
                80..=89 => { "eighty" },
                90..=99 => { "ninety" },
                _ => { "" }
            }.to_string();

            if n % 10 == 0 {
                base
            } else {
                base + &" ".to_string() + &get(n%10)
            }
        },
        100..=999 => {
            let mut base = get(n / 100);

            base = base + &" hundred".to_string();

            if n % 100 > 0 {
                base = base + &" and ".to_string() + &get(n % 100);
            }

            base

        },
        1000 => {
            "one thousand".to_string()
        }
        _ => "".to_string()
    }
}
fn main() {

    let mut n = 1000;
    let mut c = 0;

    while n > 0 {
        println!("{:?}", get(n));
        c += get(n)
            .chars()
            .filter(|x| x != &' ')
            .count();

        n -= 1;
    }

    println!("{:?}", c);
}