use std::fs::read_to_string;
#[derive(Debug, Clone, Copy)]
struct Segment {
    start: u32,
    end: u32,
}

impl TryFrom<&str> for Segment {
    type Error = color_eyre::Report;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let nums = s
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if nums.len() != 2 {
            return Err(color_eyre::eyre::eyre!("Invalid numbers"));
        }

        Ok(Segment {
            start: nums[0],
            end: nums[1]
        })
    }
}

impl Segment {
    fn includes(&self, o: &Self) -> bool {
        return o.start >= self.start && o.end <= self.end
    }
    fn overlaps(&self, o: &Self) -> bool {
        return (self.start >= o.start && self.start <= o.end) || (self.end >= o.start && self.end <= o.end)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    seg1: Segment,
    seg2: Segment
}

impl TryFrom<&str> for Pair {
    type Error = color_eyre::Report;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split(",")
            .map(|x| x.try_into().unwrap())
            .collect::<Vec<Segment>>();

        if parts.len() != 2 {
            return Err(color_eyre::eyre::eyre!("Invalid numbers when creating Pairs"));
        }

        Ok(Pair {
            seg1: parts[0],
            seg2: parts[1],
        })
    }
}

impl Pair {
    fn includes(&self) -> bool {
        return self.seg1.includes(&self.seg2) || self.seg2.includes(&self.seg1)
    }
    fn overlaps(&self) -> bool {
        return self.seg1.overlaps(&self.seg2) || self.seg2.overlaps(&self.seg1)
    }
}

fn run(fp: &str) -> (usize, usize) {
    let contents = read_to_string(fp).unwrap();
    let pairs = contents.lines().map(|x| x.try_into().unwrap()).collect::<Vec<Pair>>();

    (
        pairs.iter().filter(|x| x.includes()).count(),
        pairs.iter().filter(|x| x.overlaps()).count()
    )
}

fn main() {
    let (fisrt, second) = run("input.txt");

    println!("#1 {fisrt}");
    println!("#2 {second}");
}

#[test]
fn input_test() {
    let r = run("input_test.txt");

    assert_eq!(2, r.0);
    assert_eq!(4, r.1);
}