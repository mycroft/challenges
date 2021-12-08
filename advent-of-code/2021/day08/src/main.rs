fn solve_puzzle(words: Vec<&str>) -> Vec<char> {
    let words = words.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let letters_1 = words.iter().find(|x| x.len() == 2).unwrap();
    let letters_7 = words.iter().find(|x| x.len() == 3).unwrap();
    let letters_4 = words.iter().find(|x| x.len() == 4).unwrap();
    let letters_8 = words.iter().find(|x| x.len() == 7).unwrap();

    let mut known_segments = [' ';7];
    
    /*
 aaaa
b    c
b    c
 dddd 
e    f
e    f
 gggg
     */

    // upper segment: a
    known_segments[0] = *letters_7.iter().find(|&x| !letters_1.contains(x)).unwrap();
    
    // segment f & c: Appears 9 times out of 10
    let count_0 = words.iter().filter(|&x| x.contains(&letters_1[0])).count();
    let count_1 = words.iter().filter(|&x| x.contains(&letters_1[1])).count();

    if count_0 == 9 {
        known_segments[5] = letters_1[0];
        known_segments[2] = letters_1[1];
    } else if count_1 == 9 {
        known_segments[5] = letters_1[1];
        known_segments[2] = letters_1[0];
    } else {
        unreachable!();
    }

    // segments b & d: b appears 6 times, d appears 7 times
    let remaining_letters = letters_4.iter().filter(|&&x| !known_segments.contains(&x)).cloned().collect::<Vec<char>>();
    let count_0 = words.iter().filter(|&x| x.contains(&remaining_letters[0])).count();
    let count_1 = words.iter().filter(|&x| x.contains(&remaining_letters[1])).count();

    if count_0 == 6 {
        known_segments[1] = remaining_letters[0];
        known_segments[3] = remaining_letters[1];
    } else if count_1 == 6 {
        known_segments[1] = remaining_letters[1];
        known_segments[3] = remaining_letters[0];
    } else {
        unreachable!();
    }

    // segments e & g: e appears 4 times, g appears 7 times
    let remaining_letters = letters_8.iter().filter(|&&x| !known_segments.contains(&x)).cloned().collect::<Vec<char>>();
    let count_0 = words.iter().filter(|&x| x.contains(&remaining_letters[0])).count();
    let count_1 = words.iter().filter(|&x| x.contains(&remaining_letters[1])).count();

    if count_0 == 4 {
        known_segments[4] = remaining_letters[0];
        known_segments[6] = remaining_letters[1];
    } else if count_1 == 4 {
        known_segments[4] = remaining_letters[1];
        known_segments[6] = remaining_letters[0];
    } else {
        unreachable!();
    }

    // println!("KNOWN: {:?}", known_segments);

    known_segments.to_vec()
}

fn str_to_words(part: &str) -> Vec<&str> {
    part
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .filter(|x| x.len() >= 2)
        .collect::<Vec<&str>>()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut output_1478 = 0;
    let mut output_total = 0;

    for line in &lines {
        let parts = line.split('|').collect::<Vec<&str>>();

        let left_digits = str_to_words(parts[0]);
        let output_digits = str_to_words(parts[1]);

        output_1478 += output_digits
            .iter()
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();

        let solutions = solve_puzzle(left_digits);

        let output_number = output_digits
            .iter()
            .map(|x| to_number(x, &solutions))
            .fold(0, |x, v| x * 10 + v);

        output_total += output_number;
    }

    println!("#1: {}", output_1478); // 390
    println!("#2: {}", output_total);
}

fn to_number(z: &str, solutions: &[char]) -> i32 {
    let numbers = vec![
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6]
    ];

    // I got a word. Now I need to find the correct index of numbers according
    // to the letters in the solution & the word.

    let letters = numbers
        .iter()
        .map(|x| {
            let mut sub_array = x.iter().map(|y| solutions[*y as usize]).collect::<Vec<char>>();
            sub_array.sort_unstable();
            sub_array
        })
        .collect::<Vec<Vec<char>>>();

    let mut our_letters = z.chars().collect::<Vec<char>>();
    our_letters.sort_unstable();

    for (i, item) in (&letters).iter().cloned().enumerate() {
        if item == our_letters {
            return i as i32;
        }
    }

    unreachable!();
}
