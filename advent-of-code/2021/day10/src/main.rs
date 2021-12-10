fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut total_score_1 = 0;
    let mut scores_2 = vec![];

    for line in lines {
        let score = get_score(line);
        let mut score_2: u128 = 0;

        total_score_1 += match score.1 {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        };

        score.2.iter().map(|c| {
            score_2 *= 5;
            score_2 += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            };
        }).count();

        if score_2 > 0 {
            scores_2.push(score_2);
        }
    }

    scores_2.sort_unstable();
    let total_score_2 = scores_2[(scores_2.len()-1) / 2];

    println!("#1 {}", total_score_1);
    println!("#2 {}", total_score_2);
}

fn get_score(s: &str) -> (bool, char, Vec<char>) {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' => {
                stack.push(')');
            },
            '<' => {
                stack.push('>');
            },
            '{' => {
                stack.push('}');
            },
            '[' => {
                stack.push(']');
            }
            x => {
                if stack.is_empty() {
                    // println!("closing with '{}' state: {:?}", x, stack);
                    return (false, x, vec![]);
                }
                let p = stack.pop().unwrap();
                if p != x {
                    // println!("closing with '{}' state: {:?}", x, stack);
                    return (false, x, vec![]);
                }
            }
        }
    }

    stack.reverse();

    (true, ' ', stack)
}

#[test]
fn tests() {
    assert_eq!(
        (true, ' ', "}}]])})]".chars().collect()),
        get_score("[({(<(())[]>[[{[]{<()<>>")
    );

    assert_eq!((false, '}', vec![]), get_score("{([(<{}[<>[]}>{[]{[(<()>"));
    assert_eq!((false, ')', vec![]), get_score("[[<[([]))<([[{}[[()]]]"));
    assert_eq!((false, ']', vec![]), get_score("[{[{({}]{}}([{[{{{}}([]"));
    assert_eq!((false, ')', vec![]), get_score("[<(<(<(<{}))><([]([]()"));
    assert_eq!((false, '>', vec![]), get_score("<{([([[(<>()){}]>(<<{{"));
}


