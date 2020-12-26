use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn score(data: &HashMap<String, HashMap<String, i32>>, visited: &mut Vec<String>) -> i32 {
	if visited.len() == data.len() {
		let mut score : i32 = 0;

		if !(visited[visited.len()-1] == "me" || visited[0] == "me") {
			score = *data.get(&visited[visited.len()-1]).unwrap().get(&visited[0]).unwrap();
			score += *data.get(&visited[0]).unwrap().get(&visited[visited.len()-1]).unwrap();
		}


		for i in 0..visited.len() - 1 {
			if !(visited[i] == "me" || visited[i+1] == "me") {
				score += *data.get(&visited[i]).unwrap().get(&visited[i+1]).unwrap();
				score += *data.get(&visited[i+1]).unwrap().get(&visited[i]).unwrap();
			}
		}

		return score;
	}

	let mut current_score : i32 = 0;

	for(key, _person) in data {
		if visited.iter().any(|x| x == key) {
			continue;
		}

		visited.push(key.to_string());

		let score = score(data, visited);
		if score > current_score {
			current_score = score;
		}

		visited.pop();
	}

	current_score
}

fn main() {
	let content = fs::read_to_string("input.txt")
		.expect("Something went wrong reading the file");

	let re = Regex::new(r"^(.*) would (lose|gain) (\d+) happiness units by sitting next to (.*)\.$").unwrap();

	let lines = content.lines();
	let mut users : HashMap<String, HashMap<String, i32>> = HashMap::new();

	for line in lines {
		let cap = re.captures(line).unwrap();
		let mut value : i32 = cap[3].parse().unwrap();

		if &cap[2] == "lose" {
			value *= -1;
		}

		// println!("{:?} / {:?} / {:?}", &cap[1], value, &cap[4]);

		users.entry(String::from(&cap[1])).or_insert(HashMap::new());
		users.get_mut(&String::from(&cap[1])).unwrap().insert(
			String::from(&cap[4]), value
		);
	}

	println!("Part #1: {:?}", score(&users, &mut Vec::new()));

	users.entry("me".to_string()).or_insert(HashMap::new());

	println!("Part #2: {:?}", score(&users, &mut Vec::new()));
}
