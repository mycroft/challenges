use std::collections::{HashSet, VecDeque};
use intcode::{parse,Machine};

fn main() {
    let code = parse("input.txt");
    let mut machines = Vec::new();
    let mut queues: Vec<VecDeque<isize>> = Vec::new();

    let mut sent_values = HashSet::new();

    let mut step1_result_shown = false;
    let mut nat_x = 0;
    let mut nat_y = 0;

    for n in 0..50 {
        let machine = Machine::new(&code);
        queues.push([n].into());
        machines.push(machine);
    }

    loop {
        for n in 0..50 {
            // println!("running machine {:?} queue: {:?}", n, queues[n]);
            // run machine; if there is some input (1 or 2 bytes), send them.
            if queues[n].len() == 1 {
                machines[n].add_input(queues[n].pop_front().unwrap());
            } else if queues[n].len() >= 2 {
                machines[n].add_input(queues[n].pop_front().unwrap());
                machines[n].add_input(queues[n].pop_front().unwrap());
            } else {
                machines[n].add_input(-1);
            }

            // run machine
            machines[n].run();

            // machine stopped. Check if there is some output and dispatch
            let output = machines[n].get_output();
            // println!("output: {output:?}");
            machines[n].clean_output();

            for slice in output.chunks(3) {
                if slice[0] == 255 {
                    nat_x = slice[1];
                    nat_y = slice[2];
                    if !step1_result_shown {
                        println!("#1 {}", slice[2]); // 21089
                        step1_result_shown = true;
                    }
                    // write into nat.
                } else {
                    // println!("Got packet for machine {}: {} {}", slice[0], slice[1], slice[2]);
                    queues[slice[0] as usize].push_back(slice[1]);
                    queues[slice[0] as usize].push_back(slice[2]);    
                }
            }
        }

        // NAT
        let network_is_idling = (0usize..50).filter(|idx| queues[*idx].is_empty()).count() == 50;
        if network_is_idling && step1_result_shown {
            // println!("Network is idling. Sending {nat_x} {nat_y}");
            if sent_values.contains(&nat_y) {
                println!("#2 {nat_y}"); // 16658
                return;
            }
            sent_values.insert(nat_y);
            machines[0].add_input(nat_x);
            machines[0].add_input(nat_y);
        }
    }

}