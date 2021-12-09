fn is_valid(n: i32) -> (bool, bool) {
    // 2 adjacents digits must be the same.
    // Going from left to right, the digits never decrease; they only ever increase or stay the same.

    let mut current_number = 0;
    let mut current_number_iteration = 0;
    let mut old = 0;
    let mut has_couple = false;
    let mut has_only_couple = false;

    for i in (0..6).rev() {
        let c = (n / (10i32.pow(i))) % 10;

        if c < old {
            return (false, false);
        }

        if c == old {
            has_couple = true;
        }

        old = c;

        if current_number == c {
            current_number_iteration += 1;
        } else if current_number != c {
            if current_number_iteration == 1 {
                // single couple
                has_only_couple = true;
            }

            current_number_iteration = 0;
            current_number = c;
        }

        if i == 0 && current_number_iteration == 1 {
            has_only_couple = true;
        }
    }

    (has_couple, has_only_couple)
}

fn main() {
    let min = 130254;
    let max = 678275;

    let mut res_1 = 0;
    let mut res_2 = 0;

    for i in min..=max {
        let res = is_valid(i);
        if res.0 {
            res_1 += 1;
        }
        if res.1 {
            res_2 += 1;
        }
    }

    println!("#1 {}", res_1);
    println!("#2 {}", res_2);
}
