use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn get_weight(roads: &HashMap<(String, String), u32>, from: String, to: String) -> u32 {
	if roads.contains_key(&(from.to_string(), to.to_string())) {
		roads[&(from, to)]
	} else {
		roads[&(to, from)]
	}
}

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let lines = content
        .split("\n")
        .filter(|x| x != &"");

    let re = Regex::new(r"^([^ ]*) to ([^ ]*) = (\d+)$").unwrap();

    let mut roads = HashMap::new();
    let mut cities : Vec<String> = Vec::new();

    for line in lines {
        assert!(re.is_match(line));

        let cap = re.captures(line).unwrap();

        roads.insert(
            (String::from(&cap[1]), String::from(&cap[2])),
            cap[3].parse::<u32>().unwrap()
        );

        if !cities.iter().any(|city| city == &String::from(&cap[1])) {
            cities.push(String::from(&cap[1]));
        }

        if !cities.iter().any(|city| city == &String::from(&cap[2])) {
            cities.push(String::from(&cap[2]));
        }
    }

    let mut possibilities : Vec<Vec<String>> = Vec::new();
    for city in &cities {
    	possibilities.push(vec![city.to_string()]);
    }

    let mut weights = Vec::new();
    for i in possibilities.iter() {
    	weights.push(0);
    }

    loop {
    	let mut added = false;
   		let mut new_possibles : Vec<Vec<String>> = Vec::new();
  		let mut new_weigths = Vec::new();
 
    	for city in &cities {
    		for index in 0..possibilities.len() {
    			let possible = &possibilities[index];

    			if possible.iter().any(|inner| inner == city) {
    				continue;
    			}

    			let mut new_path = possible.clone();
    			new_path.push(city.to_string());

    			let mut new_weigth = weights[index] + get_weight(&roads, city.to_string(), new_path[possible.len()-1].to_string());

    			new_possibles.push(new_path);
    			new_weigths.push(new_weigth);

    			added = true;
    		}
    	}

    	if !added {
    		break;
    	}

    	weights = new_weigths;
    	possibilities = new_possibles;
    }

    let mut min_value = 0;
    let mut max_value = 0;

    for v in weights {
    	if min_value == 0 {
    		min_value = v;
    		continue;
    	} 

    	if min_value > v {
    		min_value = v;
    	}

    	if max_value < v {
    		max_value = v;
    	}
    }

    println!("min:{:?} max:{:?}", min_value, max_value);
}
