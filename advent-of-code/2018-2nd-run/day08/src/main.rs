use std::fs;

fn pop_front(numbers: &mut Vec<usize>) -> usize {
    let number = numbers[0];
    numbers.remove(0);

    number
}

fn get_metadata_sum(numbers: &mut Vec<usize>) -> usize {
    let num_childs : usize = pop_front(numbers);
    let num_metadata_entries : usize = pop_front(numbers);
    let mut metadata_sum : usize = 0;

    // println!("Node has {} children & {} metadata entries", num_childs, num_metadata_entries);
    // println!("Remaining: {:?}", numbers);

    for _ in 0..num_childs {
        metadata_sum += get_metadata_sum(numbers);
    }

    for _ in 0..num_metadata_entries {
        metadata_sum += pop_front(numbers); 
    }

    metadata_sum
}

fn get_node_value(numbers: &mut Vec<usize>) -> usize {
    let mut node_value : usize = 0;
    let num_childs : usize = pop_front(numbers);
    let num_metadata_entries : usize = pop_front(numbers);

    let mut sub_node_values : Vec<usize> = vec![];

    for _ in 0..num_childs {
        sub_node_values.push(get_node_value(numbers));
    }

    for _ in 0..num_metadata_entries {
        let idx = pop_front(numbers);
        if num_childs == 0 {
            node_value += idx;
            continue;
        }

        if idx == 0 || idx > sub_node_values.len() {
            continue;
        }

        node_value += sub_node_values[idx - 1];
    }

    node_value
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut numbers = contents
        .split_whitespace()
        .filter(|&x| x != " ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut numbers_copy = numbers.clone();

    let total = get_metadata_sum(&mut numbers);
    let root_node_value = get_node_value(&mut numbers_copy);

    // println!("{:?}", numbers);

    println!("#1: {}", total);
    println!("#2: {}", root_node_value);
}
