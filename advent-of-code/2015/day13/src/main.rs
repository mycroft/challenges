use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn score<'a>(
	data: &HashMap<(&'a str, &'a str), i32>,
	everyone: &Vec<&'a str>,
	visited: &mut Vec<&'a str>) -> i32
{
	if visited.len() == everyone.len() {
		let mut score : i32;

		score = *data.get(&(visited[visited.len()-1], visited[0])).unwrap();
		score += *data.get(&(visited[0], visited[visited.len()-1])).unwrap();

		for i in 0..visited.len() - 1 {
			score += *data.get(&(visited[i], visited[i+1])).unwrap();
			score += *data.get(&(visited[i+1], visited[i])).unwrap();
		}

		return score;
	}

	let mut current_score = None;

	for key in everyone.iter() {
		if visited.iter().any(|x| x == key) {
			continue;
		}

		visited.push(key);

		let score = score(data, everyone, visited);
		current_score = match current_score {
			None => Some(score),
			Some(value) => {
				if score > value { Some(score) } else { Some(value) }
			}
		};

		visited.pop();
	}

	current_score.unwrap()
}

fn main() {
	let content = fs::read_to_string("input.txt")
		.expect("Something went wrong reading the file");

	let re = Regex::new(r"^(.*) would (lose|gain) (\d+) happiness units by sitting next to (.*)\.$").unwrap();

	let lines = content.lines();
	let mut users : HashMap<(&str, &str), i32> = HashMap::new();
	let mut everyone : Vec<&str> = Vec::new();

	for line in lines {
		let cap = re.captures(line).unwrap();
		let mut value : i32 = cap[3].parse().unwrap();

		if &cap[2] == "lose" {
			value *= -1;
		}

		if !everyone.iter().any(|x| *x == cap.get(1).unwrap().as_str()) {
			everyone.push(cap.get(1).unwrap().as_str());
		}

		// println!("{:?} / {:?} / {:?}", &cap[1], value, &cap[4]);
		users.insert(
			(cap.get(1).unwrap().as_str(), cap.get(4).unwrap().as_str()),
			value
		);
	}

	println!("Part #1: {:?}", score(&users, &everyone, &mut Vec::new()));

	for person in everyone.iter() {
		users.insert((&"me", person), 0);
		users.insert((person, &"me"), 0);
	}
	everyone.push(&"me");

	// println!("{:?}", users);

	println!("Part #2: {:?}", score(&users, &everyone, &mut Vec::new()));
}
