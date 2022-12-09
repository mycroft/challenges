use std::fs::read_to_string;
use std::collections::{VecDeque,HashMap};

#[derive(Debug)]
struct Entry {
    dir: HashMap<String, Entry>,
    size: usize,
}

fn compute(n: String, t: &HashMap<String, Entry>) -> (usize, HashMap<VecDeque<String>, usize>) {
    let mut total_size = 0;
    let mut r: HashMap<VecDeque<String>, usize> = HashMap::new();

    for el in t {
        if el.1.size > 0 {
            total_size += el.1.size;
        } else {
            let c = compute(el.0.to_string(), &el.1.dir);
            total_size += c.0;

            for z in c.1 {
                let mut path = z.0;
                path.push_front(n.to_string());
                r.insert(
                    path,
                    z.1,
                );
            }
        }
    }

    let mut path = VecDeque::new();
    path.push_front(n);

    r.insert(
        path,
        total_size,
    );

    (total_size, r)
}

fn parse(s: &str) -> (usize, usize) {
    let contents = read_to_string(s).expect("could not open file");
    let lines : Vec<&str> = contents.lines().collect();

    let mut current_path = VecDeque::<String>::new();

    let mut root: HashMap<String, Entry> = HashMap::new();
    let mut current_dir = &mut root;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();

        if line.starts_with("$ cd ") {
            if parts[2] == ".." {
                current_path.pop_back();
            } else if parts[2] == "/" {
                current_path = VecDeque::<String>::new();
            } else {
                current_path.push_back(parts[2].to_string());
            }

            current_dir = &mut root;
            
            for el in current_path.iter() {
                current_dir = &mut current_dir.get_mut(el).unwrap().dir;
            }
        
        } else if line.starts_with("$ ls") {
            // skip
        } else if line.starts_with("dir ") {
            // create sub entry
            current_dir.insert(parts[1].to_string(),Entry{
                dir: HashMap::new(),
                size: 0
            });
        } else {
            current_dir.insert(parts[0].to_string(), Entry{
                dir: HashMap::new(),
                size: parts[0].parse::<usize>().unwrap()
            });
        }
    }

    // Navigate into the tree to have to have size of repertories

    let res = compute("".to_string(), &root);
    let fs_size = 70000000;
    let req_size = 30000000;
    let mut req_path = VecDeque::new();
    req_path.push_front("".to_string());
    let total_free = fs_size - res.1.get(&req_path).unwrap();
    let req_space = req_size - total_free;

    let mut res_1 = 0;
    let mut res_2 = fs_size;

    for el in &res.1 {
        if el.1 <= &100000 {
            res_1 += el.1;
        }

        if el.1 > &req_space && el.1 < &res_2 {
            res_2 = *el.1;
        }
    }

    (res_1, res_2)
}

fn main() {
    let res = parse("input.txt");

    println!("#1 {}", res.0);
    println!("#2 {}", res.1);

}

#[test]
fn test() {
    let res = parse("input.txt_test");
    assert_eq!((95437, 24933642), res);
}
