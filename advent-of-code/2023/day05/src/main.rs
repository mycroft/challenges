use std::fs;
use std::collections::{HashSet,HashMap};
use std::cmp::min;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct SeedRange {
    range_start: usize,
    len_range: usize
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Segment {
    destination_range_start: usize,
    source_range_start: usize,
    len_range: usize
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to read");
    let lines: Vec<&str> = contents.lines().collect();

    let seeds = lines[0].split(": ").nth(1).unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<HashSet<usize>>();
    let mut seeds_ranges : HashSet<SeedRange> = HashSet::new();

    let mut maps: HashMap<String, HashSet<Segment>> = HashMap::new();

    let mut sections : Vec<String> = Vec::new();

    let mut new_section = true;
    let mut section_label= String::new();

    for line in lines {
        if line.starts_with("seeds: ") {
            let ranges = line.split(": ").nth(1).unwrap();
            let mut c = 0;

            let mut range_start: usize = 0;
            let mut range_len: usize = 0;

            for range_num in ranges.split(' ') {
                if c == 0 {
                    range_start = range_num.parse::<usize>().unwrap();
                    c = 1;
                } else {
                    range_len = range_num.parse::<usize>().unwrap();
                    seeds_ranges.insert(SeedRange { range_start: range_start, len_range: range_len });
                    c = 0;
                }
            }

            seeds_ranges.insert(SeedRange { range_start: range_start, len_range: range_len });
        }
        if line.is_empty() || line.starts_with("seeds:") {
            new_section = true;
            continue;
        }

        if new_section {
            section_label = String::from(line.split(' ').next().unwrap());

            maps.insert(section_label.clone(), HashSet::new());
            sections.push(section_label.clone());
            new_section = false;
            continue;
        }

        let set = maps.get_mut(&section_label).unwrap();
        let map_elements = line.split(' ').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        set.insert(Segment{
            destination_range_start: *map_elements.get(0).unwrap() as usize,
            source_range_start: *map_elements.get(1).unwrap() as usize,
            len_range: *map_elements.get(2).unwrap() as usize,
        });
    }

    let mut min_location: Option<usize> = None;
    
    for seed in seeds {
        let mut current_location = seed;
        for section in &sections {
            let current_maps = maps.get(section).unwrap();
            for current_map in current_maps {
                if current_map.source_range_start <= current_location && current_map.source_range_start + current_map.len_range > current_location {
                    current_location = current_map.destination_range_start + (current_location - current_map.source_range_start);
                    break;
                }
            }
        }
        println!("current location: seed:{} => {}", seed, current_location);

        if min_location.is_none() || min_location.unwrap() > current_location {
            min_location = Some(current_location);
        }
    }

    // part 2
    let mut min_location_p2 : Option<usize> = None;
    let mut all_seeds_ranges = seeds_ranges.clone();
    for my_range in all_seeds_ranges {
        let mut ranges: HashSet<SeedRange> = HashSet::new();
        ranges.insert(my_range);

        for section in &sections {
            let new_ranges = get_ranges(&maps, section, &ranges);
            if !new_ranges.is_empty() {
                ranges = new_ranges;
            }

            println!("Section: {} ranges: {:?}", section, ranges);
        }

        for range in ranges {
            if min_location_p2.is_none() || min_location_p2.unwrap() > range.range_start {
                min_location_p2 = Some(range.range_start);
            }
        }
    }

    println!("#1 {}", min_location.unwrap());
    println!("#2 {}", min_location_p2.unwrap());

}

fn get_ranges(maps: &HashMap<String, HashSet<Segment>>, current_section: &String, ranges: &HashSet<SeedRange>) -> HashSet<SeedRange> {
    let mut hs = HashSet::new();
    let mut ranges_to_handle : Vec<SeedRange> = Vec::new();
    let current_maps = maps.get(current_section).unwrap();

    for range in ranges {
        ranges_to_handle.push(range.clone());
    }
    
    while !ranges_to_handle.is_empty() {
        let range = ranges_to_handle.pop().unwrap();
        let mut matched = false;

        for current_map in current_maps {
            println!("current {}->{} (len:{}) prop_source {}->{} prop_dest {}->{}",
                range.range_start,
                range.range_start + range.len_range,
                range.len_range,
                current_map.source_range_start,
                current_map.source_range_start + current_map.len_range,
                current_map.destination_range_start,
                current_map.destination_range_start + current_map.len_range,
            );

            // check if any overlap
            if range.range_start + range.len_range <= current_map.source_range_start {
                println!("No overlap (range is at left); continuing...");
                continue;
            }

            if range.range_start + range.len_range > current_map.source_range_start + current_map.len_range {
                println!("No overlap (range is at right); continuing...");
                continue;
            }

            let mut cut: usize = 0;

            // check if there is a section before current_map
            if range.range_start < current_map.source_range_start {
                // split this section and push it into ranges_to_handle
                let new_range = SeedRange{
                    range_start: range.range_start,
                    len_range: current_map.source_range_start - range.range_start,
                };

                cut += new_range.len_range;

                println!("(left) New split range: {:?} (cut:{})", new_range, cut);

                ranges_to_handle.push(new_range);
            }

            // check if there is a section after
            if range.range_start + range.len_range > current_map.source_range_start + current_map.len_range {
                // split this section and push it into ranges_to_handle
                let new_range = SeedRange{
                    range_start: current_map.source_range_start + current_map.len_range,
                    len_range: (range.range_start + range.len_range) - (current_map.source_range_start + current_map.len_range),
                };

                cut += new_range.len_range;

                println!("(right) New split range: {:?} (cut:{})", new_range, cut);

                ranges_to_handle.push(new_range);
            }

            println!("len:{} cut:{}", range.len_range, cut);

            // overlapping section
            let new_range = SeedRange{
                range_start: current_map.destination_range_start + (range.range_start - min(current_map.source_range_start, range.range_start)),
                len_range: range.len_range - cut,
            };

            hs.insert(new_range);
            matched = true;
        }

        if matched {
            continue;
        }

        // nothing was found for this range; returns it as it.
        hs.insert(range.clone());
    }

    return hs;
}