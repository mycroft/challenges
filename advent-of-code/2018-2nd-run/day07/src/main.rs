use std::fs;

fn can_finish(c: char, vectors: &Vec<(char, char)>, finished: &Vec<char>) -> bool {
    for tuple in vectors {
        if tuple.1 == c {
            if !finished.contains(&tuple.0) {
                return false;
            }
        }
    }

    true
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    // parameters
    let num_workers = 5; // 2 for test, 5 for real
    let task_delay = 60; // 0 for test, 60 for real

    let mut vectors : Vec<(char, char)> = vec![];

    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        vectors.push(
            (
                parts[1].chars().nth(0).unwrap(),
                parts[7].chars().nth(0).unwrap(),
            )
        );
    }

    let mut finished : Vec<char> = vec![];
    let mut next_letters : Vec<char> = vec![];

    loop {
        let mut can_be_done : Vec<char> = vec![];
    
        for vector in &vectors {
            if finished.contains(&vector.0) {
                continue;
            }

            if can_finish(vector.0, &vectors, &finished){
                if !can_be_done.contains(&vector.0) {
                    can_be_done.push(vector.0);
                }
                // println!("Can finish: {}", vector.0);
            }
        }

        if can_be_done.len() == 0 {
            // Last letter in next list will complete the list.
            finished.push(*next_letters.last().unwrap());
            break;
        }

        can_be_done.sort();
        let c = can_be_done[0];

        // Find all letters that can be completed with this letter
        for vector in &vectors {
            if vector.0 == c {
                next_letters.push(vector.1);
            }
        }

        // println!("Will finish: {} (next in row: {:?}", c, next_letters);
        finished.push(c);
    }

    println!("#1: {}", String::from(finished.iter().collect::<String>()));

    // Part 2
   
    let mut second : usize = 0;
    let mut finished_p2 : Vec<char> = vec![];
    let mut workers = vec![Task{task: ' ', finished_at: 0}; num_workers];
    let mut next_letters : Vec<char> = vec![];

    loop {
        // Check if any task(s) is(are) over.
        for task in workers.iter_mut() {
            if task.task != ' ' && task.finished_at == second {
                finished_p2.push(task.task);

                // println!("Task {} finished at second {}", task.task, second);

                task.finished_at = 0;
                task.task = ' ';
            }
        }

        // Find if there is any worker available
        let mut available : Option<usize> = None;
        for (id, task) in workers.iter().enumerate() {
            if task.finished_at == 0 {
                available = Some(id);
                // println!("Worker {} is available for work", id);
                break;
            }
        }

        if available == None {
            // No one's available.
            // println!("No one's available now. ({:?})", workers);
            second += 1;
            continue;
        }

        // Check if any task can be started
        let mut can_be_done : Vec<char> = vec![];
        for vector in &vectors {
            if finished_p2.contains(&vector.0) || workers.iter().any(|t| t.task == vector.0) {
                continue;
            }

            if can_finish(vector.0, &vectors, &finished_p2){
                if !can_be_done.contains(&vector.0) {
                    can_be_done.push(vector.0);
                }
                // println!("Can finish: {}", vector.0);
            }
        }

        if can_be_done.len() == 0 && finished_p2.len() == finished.len() - 1 {
            // println!("No more work to be done.");
            // last letter?
            // println!("{:?}", next_letters);

            let c = next_letters.last().unwrap();
            let task_time : usize = *c as usize - 'A' as usize + 1;
            second += task_delay + task_time;

            break;
        }

        if can_be_done.len() == 0 {
            // println!("Worker available but no work to do.");
            second += 1;
            continue;
        }

        // println!("{} {} {}", can_be_done.len(), finished_p2.len(), finished.len());
        // println!("{:?}", finished_p2);

        // Assing work if any.
        can_be_done.sort();
        let c = can_be_done[0];

        // Find all letters that can be completed with this letter
        for vector in &vectors {
            if vector.0 == c {
                next_letters.push(vector.1);
            }
        }

        let task_time : usize = c as usize - 'A' as usize + 1;
        workers[available.unwrap()] = Task{task: c, finished_at: second + task_delay + task_time };
    }

    println!("#2: {}", second);
}

#[derive(Clone, Debug)]
struct Task {
    task: char,
    finished_at: usize
}