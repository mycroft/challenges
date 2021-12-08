#[derive(Clone, Debug)]
struct Point(Vec<i32>);

impl Point {
    fn manhattan(&self, other: &Point) -> i32 {
        (0..4).map(|x| {
            (self.0[x] - other.0[x]).abs()
        }).sum()
    }

    fn fit(&self, constellation: &Vec<Point>) -> bool {
        for star in constellation {
            if self.manhattan(star) <= 3 {
                return true;
            }
        }

        false
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let stars = contents
        .lines()
        .map(|x| x.trim())
        .map(|x| {
            let z = x
                .split(',')
                .map(|y| y
                    .parse::<i32>()
                    .unwrap()
                )
                .collect::<Vec<i32>>();
                Point(z)
            }
        )
        .collect::<Vec<Point>>();

    let mut constellations = vec![];

    for star in &stars {
        // check in which constellations the star could go in.
        let mut matching = vec![];
        let mut notmatching = vec![];

        for constellation in &constellations {
            if star.fit(&constellation) {
                matching.push(constellation.clone());
            } else {
                notmatching.push(constellation.clone());
            }
        }

        if matching.is_empty() {
            notmatching.push(vec![star.clone()]);
            constellations = notmatching;
            continue;
        }

        // Merge all matching
        let mut new_constellation = vec![star.clone()];
        for matching_el in matching.iter_mut() {
            new_constellation.append(matching_el);
        }

        notmatching.push(new_constellation);
        constellations = notmatching;
        continue;
    }

    println!("{}", constellations.len());
}
