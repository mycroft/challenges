use std::fs;
use std::collections::HashMap;

struct Memory {
	values : HashMap<String, u16>,
}

impl Memory {
	fn has(&self, name : String) -> bool {
		match name.parse::<u16>() {
			Ok(_) => {
				return true;
			},
			_ => {}
		}


		match self.values.get(&name) {
			None => false,
			_ => true,
		}
	}

	fn get(&mut self, name : String) -> u16 {
		// check that name is a string, not numeric.
		match name.parse::<u16>() {
			Ok(val) => {
				return val;
			},
			_ => {}
		}

		match self.values.get(&name) {
			None => 0,
			Some(val) => *val,
		}
	}

	fn set(&mut self, name : String, val : u16) {
		if self.has(name.to_string()) {
			return;
		}
		self.values.insert(name, val);
	}

	fn reset(&mut self) {
		self.values = HashMap::new();
	}

	fn solve(&mut self, lines : &Vec<String>) -> u16 {
		let mut missing = true;

		while missing {
			missing = false;

			for line in lines {
				let op : Vec<&str> = line.split(" ").collect();
				let op_count = op.len();

				match op_count {
					3 => {
						// a -> b
						if self.has(op[0].to_string()) {
							let val = self.get(op[0].to_string());
							self.set(op[2].to_string(), val);
						} else {
							missing = true;
						}
					},
					4 => {
						// NOT a -> b
						if self.has(op[1].to_string()) {
							let val = !self.get(op[1].to_string());
							self.set(op[3].to_string(), val);						
						} else {
							missing = true;
						}
					},
					5 => {
						if !self.has(op[0].to_string()) || !self.has(op[2].to_string()) {
							missing = true;
							continue;
						}

						let arg1 = self.get(op[0].to_string());
						let arg2 = self.get(op[2].to_string());
						let operator = op[1];
						let dest = op[4].to_string();

						match operator {
							"OR" => {
								self.set(dest, arg1 | arg2);
							},
							"AND" => {
								self.set(dest, arg1 & arg2);
							},
							"LSHIFT" => {
								self.set(dest, arg1 << arg2);
							},
							"RSHIFT" => {
								self.set(dest, arg1 >> arg2);
							},
							_ => {
								println!("not implemented: {:?}", operator);
							}
						}
					}
					_ => {}
				};
			}
		}

		self.get("a".to_string())
	}
}



fn main() {
	let contents = fs::read_to_string("input.txt")
		.expect("Something went wrong reading the file");

	let mut mem = Memory {
		values : HashMap::new(),
	};

	let lines : Vec<String> = contents.split("\n").filter(|x| x != &"").map(|x| x.to_string()).collect();

	let res = mem.solve(&lines);

	println!("Part #1: {}", res);

	mem.reset();
	mem.set("b".to_string(), res);

	let res2 = mem.solve(&lines);

	println!("Part #2: {}", res2);
}
