pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res : Vec<String> = vec![];

    let deltas = [
        (-1i8, -1i8), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    for (c_y, line) in minefield.iter().enumerate() {
        let mut s = String::from("");

        for (c_x, ch) in line.chars().enumerate() {
            if ch == '*' {
                s.push(ch);
            } else {
                let mut c : usize = 0;
        
                for d in deltas {
                    let t_x = c_x as i8 + d.0;
                    let t_y = c_y as i8 + d.1;

                    if t_x < 0 || t_y < 0 || t_x >= line.len() as i8 || t_y >= minefield.len() as i8{
                        continue;
                    }

                    if minefield[t_y as usize].chars().collect::<Vec<char>>()[t_x as usize] == '*' {
                        c += 1;
                    }
                }

                if c == 0 {
                    s.push(' ');
                } else {
                    s.push_str(format!("{}", c).as_str());
                }
            }
        }

        res.push(s.clone());
    }

    res
}
