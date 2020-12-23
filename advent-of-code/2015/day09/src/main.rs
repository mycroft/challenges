use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;

fn get_weight(roads: &HashMap<(String, String), u32>, from: String, to: String) -> u32 {
	if roads.contains_key(&(from.to_string(), to.to_string())) {
		roads[&(from, to)]
	} else {
		roads[&(to, from)]
	}
}

fn travel(roads: &HashMap<(String, String), u32>, visited : &mut VecDeque<String>, to_visit : &mut VecDeque<String>) -> (u32, u32) {
	let mut min_value = 999;
	let mut max_value = 0;

	if to_visit.len() == 0 {
		let mut old_item = "";
		let mut weight = 0;

		for item in visited {
			if old_item == "" {
				old_item = item;
				continue;
			}

			weight += get_weight(roads, old_item.to_string(), item.to_string());

			old_item = item;
		}

		return (weight, weight);
	}	

	for _idx in 0..to_visit.len() {
		let city = to_visit.pop_front().unwrap();

		visited.push_back(city);

		let (min_val, max_val) = travel(roads, visited, to_visit);
		if min_val < min_value {
			min_value = min_val;
		}

		if max_val > max_value {
			max_value = max_val;
		}

		let city = visited.pop_back().unwrap();
		to_visit.push_back(city);
	}

	(min_value, max_value)
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let lines = content
        .split("\n")
        .filter(|x| x != &"");

    let re = Regex::new(r"^([^ ]*) to ([^ ]*) = (\d+)$").unwrap();

    let mut roads = HashMap::new();
    let mut cities : VecDeque<String> = VecDeque::new();

    for line in lines {
        assert!(re.is_match(line));

        let cap = re.captures(line).unwrap();

        roads.insert(
            (String::from(&cap[1]), String::from(&cap[2])),
            cap[3].parse::<u32>().unwrap()
        );

        if !cities.iter().any(|city| city == &String::from(&cap[1])) {
            cities.push_back(String::from(&cap[1]));
        }

        if !cities.iter().any(|city| city == &String::from(&cap[2])) {
            cities.push_back(String::from(&cap[2]));
        }
    }

    let (min, max) = travel(
    	&roads,
    	&mut VecDeque::new(),
    	&mut cities
    );

    println!("min:{} max:{}", min, max);

}
