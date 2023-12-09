use std::fs;

fn build_p1(v: &[i32]) -> i32 {
    let mut history : Vec<Vec<i32>> = Vec::new();

    history.push(v.to_owned());

    // add a new line until last line is all 0s
    loop {
        let last_line = history.last().unwrap();
        if last_line.iter().all(|&x| x == 0) {
            break;
        }
        
        let mut new_last_line = Vec::new();
        for n in 0..last_line.len() - 1 {
            new_last_line.push(last_line[n+1] - last_line[n]);
        }

        history.push(new_last_line);
    }

    // add a 0 to each history line
    for line in &mut history {
        line.push(0);
    }

    // compute new steps, from last_history - 1 to history = 0
    for n in 0..history.len() - 1 {
        let idx = history.len()-n-2;
        let idx_in_line = history[idx].len() - 1;

        let v = history[idx+1][history[idx+1].len() - 1] + history[idx][history[idx].len() - 2];

        history[idx][idx_in_line] = v;
    }

    history[0][history[0].len()-1]
}

fn build_p2(v: &[i32]) -> i32 {
    let mut history : Vec<Vec<i32>> = Vec::new();

    history.push(v.to_owned());

    // add a new line until last line is all 0s
    loop {
        let last_line = history.last().unwrap();
        if last_line.iter().all(|&x| x == 0) {
            break;
        }
        
        let mut new_last_line = Vec::new();
        for n in 0..last_line.len() - 1 {
            new_last_line.push(last_line[n+1] - last_line[n]);
        }

        history.push(new_last_line);
    }

    // add a 0 to each history line
    for line in &mut history {
        line.insert(0, 0);
    }

    // compute new steps, from last_history - 1 to history = 0
    for n in 0..history.len() - 1 {
        let idx = history.len()-n-2;

        history[idx][0] = history[idx][1] - history[idx+1][0];
    }

    history[0][0]
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to open");
    let lines = contents
        .lines()
        .map(|x| x.split(' ').map(|x| { 
            x.parse::<i32>().unwrap()
        }).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for line in lines {
        p1 += build_p1(&line);
        p2 += build_p2(&line)
    }

    println!("#1 {:?}", p1);
    println!("#2 {:?}", p2);
}
