use mkz_aoc::file;

fn solve1(matrix: &Vec<Vec<i128>>) -> i128 {
    matrix
        .iter()
        .map(|x| x.iter().max().unwrap() - x.iter().min().unwrap())
        .sum()
}

fn div(v: Vec<i128>) -> i128 {
    for i in &v {
        for j in &v {
            if i == j || i < j {
                continue;
            }
            if i % j == 0  {
                return i/j;
            }
        }
    }

    println!("ret: 0 for {:?}", v);

    0
}

fn solve2(matrix: &Vec<Vec<i128>>) -> i128 {
    matrix
        .iter()
        .map(|x| div(x.to_vec()))
        .sum()

}

fn main() {
    let matrix = file::read_to_matrix("input.txt").unwrap();

    println!("Part #1: {}", solve1(&matrix));
    println!("Part #2: {}", solve2(&matrix));
}
