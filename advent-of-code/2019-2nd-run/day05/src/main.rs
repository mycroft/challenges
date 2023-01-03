use intcode::{parse,Machine};

fn main() {
    let code = parse("input.txt");
    println!("#1 {}", Machine::create_and_run(&code, &[1].to_vec())); // 7265618
    println!("#2 {}", Machine::create_and_run(&code, &[5].to_vec())); // 7731427
}

#[test]
fn test_input() {
    let code = parse("input.txt");
    assert_eq!(
        7265618,
        Machine::create_and_run(&code, &[1].to_vec()),
    );

    assert_eq!(
        7731427,
        Machine::create_and_run(&code, &[5].to_vec()),
    );
}
