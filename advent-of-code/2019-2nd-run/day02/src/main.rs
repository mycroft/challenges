use intcode::{parse, Machine};

fn main() {
    let mut code = parse("input.txt");
    code[1] = 12;
    code[2] = 2;
    let mut machine = Machine::new(&code);
    println!("#1 {}", machine.run()); // 3706713

    for x in 0..=99 {
        for y in 0..=99 {
            let mut code = parse("input.txt");
            code[1] = x;
            code[2] = y;
            let mut machine = Machine::new(&code);

            if machine.run() == 19690720 {
                println!("#2 {}", x*100 + y); // 8609
                return;
            }
        }
    }
}

#[test]
fn test_input() {
    let mut code = parse("input.txt");
    code[1] = 12;
    code[2] = 2;
    let mut machine = Machine::new(&code);
    println!("#1 {}", machine.run()); // 3706713

    assert_eq!(
        3706713,
        machine.run()
    );
}