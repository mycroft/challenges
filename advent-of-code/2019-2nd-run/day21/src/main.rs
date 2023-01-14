use intcode::{parse,Machine};

fn display(output: &Vec<isize>) {
    for o in output {
        print!("{}", *o as u8 as char);
    }
    println!();
}

fn instructions_as_input(instructions: &Vec<&str>) -> Vec<isize> {
    let mut result = Vec::new();

    for instruction in instructions {
        instruction.chars().map(|x| result.push(x as u8 as isize)).count();
        result.push(10);
    }

    result
}

fn step1() -> isize {
    let code = parse("input.txt");
    let mut vm = Machine::new(&code);

    /*
        Use cases:

        #####.###########
        #####...#########
        #####.#..########
        #####..#.########
     */
    // if A is an hole, jump.
    // if C is an hole, jump.
    // always check D is true before jumping
    let instructions = [
        "NOT A J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "WALK"
    ].to_vec();

    for byte in instructions_as_input(&instructions) {
        vm.add_input(byte);
    }

    let res = vm.run();
    // let output = vm.get_output();
    // display(&output);

    res
}

fn step2() -> isize {
    let code = parse("input.txt");
    let mut vm = Machine::new(&code);

    let instructions = [
        "NOT C J",
        "AND D J",
        "AND H J",
        "NOT B T",
        "AND D T",
        "OR T J",
        "NOT A T",
        "OR T J",
        "RUN",
    ].to_vec();

    for byte in instructions_as_input(&instructions) {
        vm.add_input(byte);
    }

    let res = vm.run();
    // let output = vm.get_output();
    // display(&output);

    res
}

fn main() {
    println!("#1 {}", step1());
    println!("#2 {}", step2());
}
