use std::fs;
use std::collections::HashMap;

//#[derive(Eq, PartialEq)]
type F = fn(&Vec<u32>, usize, usize, usize) -> Vec<u32>;
type Z = HashMap<usize, Vec<String>>;

fn op_addr(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] + regs[i1];
    regs_copy
}
fn op_addi(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] + i1 as u32;
    regs_copy
}
fn op_mulr(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] * regs[i1];
    regs_copy
}
fn op_muli(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] * i1 as u32;
    regs_copy
}
fn op_banr(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] & regs[i1];
    regs_copy
}
fn op_bani(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] & i1 as u32;
    regs_copy
}
fn op_borr(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] | regs[i1];
    regs_copy
}
fn op_bori(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0] | i1 as u32;
    regs_copy
}
fn op_setr(regs: &Vec<u32>, i0: usize, _i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = regs[i0];
    regs_copy
}
fn op_seti(regs: &Vec<u32>, i0: usize, _i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = i0 as u32;
    regs_copy
}
fn op_gtir(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = if i0 as u32 > regs[i1] { 1 } else { 0 };
    regs_copy
}
fn op_gtri(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = if regs[i0] > i1 as u32 { 1 } else { 0 };
    regs_copy
}
fn op_gtrr(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = if regs[i0] > regs[i1] { 1 } else { 0 };
    regs_copy
}
fn op_eqir(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = if i0 as u32 == regs[i1] { 1 } else { 0 };
    regs_copy
}
fn op_eqri(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = if regs[i0] == i1 as u32 { 1 } else { 0 };
    regs_copy
}
fn op_eqrr(regs: &Vec<u32>, i0: usize, i1: usize, o: usize) -> Vec<u32> {
    let mut regs_copy = regs.clone();
    regs_copy[o] = if regs[i0] == regs[i1] { 1 } else { 0 };
    regs_copy
}

fn discover(ops: &HashMap<String, F>, op2func: &mut Z, current_operation: &Vec<usize>, before: &Vec<u32>, after: &Vec<u32>) -> usize {
    let mut num_opscodes = 0;

    for (name, op) in ops {
        let res = op(before, current_operation[1], current_operation[2], current_operation[3]);
        if *after !=  res {
            continue;
        }

        num_opscodes += 1;

        let entries = op2func.entry(current_operation[0]).or_insert(vec![]);

        if entries.iter().any(|x| x == name) {
            continue;
        }

        entries.push(
            name.clone()
        );
    }

    num_opscodes
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let lines = file_contents.lines().collect::<Vec<&str>>();

    let mut ops : HashMap<String, F> = HashMap::new();

    ops.insert("addr".to_string(), op_addr);
    ops.insert("addi".to_string(), op_addi);
    ops.insert("mulr".to_string(), op_mulr);
    ops.insert("muli".to_string(), op_muli);

    ops.insert("banr".to_string(), op_banr);
    ops.insert("bani".to_string(), op_bani);
    ops.insert("borr".to_string(), op_borr);
    ops.insert("bori".to_string(), op_bori);

    ops.insert("gtir".to_string(), op_gtir);
    ops.insert("gtri".to_string(), op_gtri);
    ops.insert("gtrr".to_string(), op_gtrr);
    ops.insert("seti".to_string(), op_seti);

    ops.insert("setr".to_string(), op_setr);
    ops.insert("eqir".to_string(), op_eqir);
    ops.insert("eqri".to_string(), op_eqri);
    ops.insert("eqrr".to_string(), op_eqrr);

    let mut op2func : Z = HashMap::new();
    let mut state_before : Vec<u32> = Vec::new();
    let mut state_after : Vec<u32>;

    let mut prog_test : Vec<Vec<usize>> = Vec::new();

    let mut current_operation : Vec<usize> = Vec::new();

    let mut number3plus = 0;

    for line in &lines {
        if line.starts_with("Before") {
            state_before = line[1+line.find('[').unwrap()..line.find(']').unwrap()]
                .split(", ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            continue;
        }

        if line.starts_with("After") {
            state_after = line[1+line.find('[').unwrap()..line.find(']').unwrap()]
                .split(", ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if discover(&ops, &mut op2func, &current_operation, &state_before, &state_after) >= 3 {
                number3plus += 1;
            }

            state_before = Vec::new();
            continue;
        }

        if line.len() == 0 {
            continue
        }
    
        current_operation = line
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if state_before.len() == 0 {
            prog_test.push(current_operation.clone());
        }
    }

    let mut known = Vec::<String>::new();
    let mut changed;

    loop {
        if op2func.iter().all(|(_x, v)| v.len() == 1) {
            break;
        }

        changed = false;

        for (_, v) in op2func.iter_mut() {
            if v.len() == 1 && known.iter().all(|s| *s != v[0]) {
                known.push(v[0].clone());
                changed = true;
            }

            if v.len() == 1 {
                continue;
            }

            for known_func in known.iter() {
                if let Some(index) = v.iter().position(|x| x == known_func) {
                    v.remove(index);
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    /*
    println!("final:");
    for k in 0..16 {
        println!("{:?} {:?}", k, op2func[&(k as usize)]);
    }
    */

    println!("Part #1: {}", number3plus);

    let mut regs : Vec<u32> = [0 as u32; 4].to_vec();

    for inst in prog_test {
        let op_name = &op2func[&inst[0]];
        let op = ops[&op_name[0]];

        regs = op(&regs, inst[1], inst[2], inst[3]);

        // println!("{:?}", regs);
    }

    println!("Part #2: {}", regs[0]);
}
