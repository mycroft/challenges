#[macro_use] extern crate scan_fmt;

fn get_pathes(ways: &[(String, String)], visited: &[String], starting_point: String, bonus: bool) -> Vec<Vec<String>> {
    if starting_point == "end" {
        return vec![vec!["end".to_string()]];
    }

    let mut pathes: Vec<Vec<String>> = vec![];

    // We consider having visited starting_point.
    let mut new_visited = visited.to_owned();
    let c : char = starting_point.chars().next().unwrap();
    if c != c.to_uppercase().next().unwrap() {
        new_visited.push(starting_point.clone());
    }

    // For each successors to starting_point, start algorithm if it doesn't in visited already
    for (from, to) in ways {
        let mut bonus= bonus;
        if from != &starting_point || (visited.contains(to) && !bonus) {
            continue;
        }

        if visited.contains(to) {
            // using bonus
            bonus = false;
        }

        let mut sub_pathes = get_pathes(ways, &new_visited, to.clone(), bonus);

        for path in sub_pathes.iter_mut() {
            path.insert(0, starting_point.clone());
            pathes.push(path.clone());
        }
    }

    pathes
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut ways = vec![];

    for line in lines {
        let (from, to) = scan_fmt!(
            line,
            "{}-{}",
            String, String
        ).unwrap();

        if from != "end" && to != "start" {
            ways.push((from.clone(), to.clone()));
        }

        if from != "start" && to != "end" {
            ways.push((to.clone(), from.clone()));
        }
    }

    let all_pathes = get_pathes(&ways, &[], "start".to_string(), false);
    println!("#1 {}", all_pathes.len());

    let all_pathes = get_pathes(&ways, &[], "start".to_string(), true);
    println!("#2 {}", all_pathes.len());
}
