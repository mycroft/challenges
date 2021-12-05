fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file to read");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut drawn_numbers : Vec<i32> = vec![];
    let mut current_matrix = vec![];
    let mut cards = vec![];

    for line in lines {
        if drawn_numbers.len() == 0 {
            drawn_numbers = line.split(",").map(|i| i.parse::<i32>().expect("int")).collect();
            continue;
        }

        if line.len() == 0 {
            if current_matrix.len() != 0 {
                cards.push(current_matrix);
            }
            current_matrix = vec![];
            continue;
        }

        let current_line : Vec<i32> = line
            .split(" ")
            .filter(|x| x.len() > 0)
            .map(|i| i.parse::<i32>()
            .expect("int")).collect();

        current_matrix.push(current_line);
    }

    if current_matrix.len() != 0 {
        cards.push(current_matrix);
    }
    
    let mut card_worn = vec![false; cards.len()];

    let mut final_score_1 = 0;
    let mut final_score = 0;
    for draw in drawn_numbers {
        for (card_number, card) in cards.iter_mut().enumerate() {
            for ln in 0..5 {
                for en in 0..5 {
                    if card[ln][en] == draw {
                        card[ln][en] = -1;
                    }
                }
            }

            if is_winning_card(card.to_vec()) && !card_worn[card_number] {
                let remaining :i32 = card
                    .iter()
                    .map(|line| line
                        .iter()
                        .filter(|&&n| n != -1)
                        .sum::<i32>()
                    )
                    .sum::<i32>();

                if final_score_1 == 0 {
                    final_score_1 = remaining * draw;
                }
                final_score = remaining * draw;

                card_worn[card_number] = true;
            }
        }
    }

    println!("#1 {}", final_score_1);
    println!("#2 {}", final_score);
}

fn is_winning_card(card: Vec<Vec<i32>>) -> bool {
    // check lines
    for ln in 0..5 {
        let mut is_winning = true;
        for en in 0..5 {
            if card[ln][en] != -1 {
                is_winning = false;
            }
        }
        if is_winning {
            return true;
        }
    }

    // check rows
    for ln in 0..5 {
        let mut is_winning = true;
        for en in 0..5 {
            if card[en][ln] != -1 {
                is_winning = false;
            }
        }
        if is_winning {
            return true;
        }
    }

    // check diags
    let mut is_winning = true;
    for ln in 0..5 {
        if card[ln][ln] != -1 {
            is_winning = false;
        }
    }
    if is_winning {
        return true;
    }

    let mut is_winning = true;
    for ln in 0..5 {
        if card[ln][4 - ln] != -1 {
            is_winning = false;
        }
    }
    if is_winning {
        return true;
    }

    false
}
