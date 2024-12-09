use std::fs;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
struct Area {
    size: usize,
    location: usize
}

#[derive(Debug, Clone)]
struct Disk {
    content: VecDeque<Option<usize>>,
    available: VecDeque<Area>,
    files: HashMap<usize, Area>,
}

impl Disk {
    fn dump(&self) -> String {
        let mut result = String::new();
        for c in &self.content {
            match c {
                Some(file_id) => result.push_str(&format!("{}", file_id)),
                None => result.push('.')
            }
        }
        result
    }

    fn find_available(&mut self, size: usize) -> Option<usize> {
        for (idx, area) in self.available.iter().enumerate() {
            if area.size >= size {
                let start_idx = area.location;
                if area.size == size {
                    self.available.remove(idx);
                } else {
                    self.available[idx].size -= size;
                    self.available[idx].location += size;
                }
                return Some(start_idx);
            }
        }
        None
    }
}


fn read_input(fp: &str) -> Disk {
    let line = fs::read_to_string(fp).expect("Cannot read file");
    
    let mut content = VecDeque::new();
    let mut available = VecDeque::new();
    let mut files = HashMap::new();

    let mut current_index = 0;
    let mut current_file_id = 0;

    let mut is_content = true;

    for c in line.chars() {
        if c == '\n' {
            continue;
        }
        let count = c.to_digit(10).unwrap() as usize;
        if is_content {
            for _idx in 0..count {
                content.push_back(Some(current_file_id));
            }

            files.insert(current_file_id, Area {
                size: count,
                location: current_index
            });

            current_index += count;
            current_file_id += 1;

        } else {
            available.push_back(Area { size: count, location: current_index });
    
            for _idx in 0..count {            
                content.push_back(None);
            }
            current_index += count;
        }

        is_content = !is_content;
    }

    Disk {
        content,
        available,
        files
    }
}

fn checksum(disk: &Disk) -> usize {
    let mut result = 0;

    for (idx, c) in disk.content.iter().enumerate() {
        match c {
            Some(file_id) => result += file_id * idx,
            None => continue
        }
    }

    result
}

fn arrange_step1(disk: &Disk) -> Disk {
    let mut idx = 0;
    let mut disk = disk.clone();

    while idx < disk.content.len() {
        if disk.content[idx].is_some() {
            idx += 1;
            continue;
        }

        disk.content[idx] = disk.content.pop_back().unwrap();

        while disk.content[disk.content.len() - 1].is_none() {
            disk.content.pop_back();
        }

        idx += 1;
    }

    disk
}

fn arrange_step2(disk: &Disk) -> Disk {
    let mut disk = disk.clone();

    let mut current_file_id = disk.files.len() - 1;
    
    loop {
        let current_file = *disk.files.get(&current_file_id).unwrap();

        let target_area = disk.find_available(current_file.size);
        if target_area.is_none() {
            if current_file_id == 0 {
                break;
            }
            current_file_id -= 1;
            continue
        }

        let target_area_idx = target_area.unwrap();

        if current_file.location < target_area_idx {
            current_file_id -= 1;
            continue;
        }

        // copy file at area
        for idx in 0..current_file.size {
            disk.content[target_area_idx + idx] = Some(current_file_id);
            disk.content[current_file.location + idx] = None;
        }

        current_file_id -= 1;
    }

    disk.clone()
}

fn solve_step1(disk: &Disk) -> usize {
    let disk = arrange_step1(disk);
    checksum(&disk)
}

fn solve_step2(disk: &Disk) -> usize {
    let disk = arrange_step2(disk);
    checksum(&disk)
}

fn main() {
    let disk = read_input("input.txt");

    let result_step1 = solve_step1(&disk);
    println!("#1: {}", result_step1);

    let result_step2 = solve_step2(&disk);
    println!("#2: {}", result_step2);
}

#[test]
fn test_solve() {
    let disk = read_input("input_test.txt");
    assert_eq!(1928, solve_step1(&disk));
    assert_eq!(2858, solve_step2(&disk));
}