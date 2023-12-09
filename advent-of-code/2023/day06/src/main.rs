
fn ways(time: i128, distance: i128) -> i128 {
    let mut c = 0;
    for n in 0..time {
        if n * (time - n) > distance {
            c += 1;
        }
    }

    c
}

fn main() {
    let mut p1 = 1;

    let games: Vec<(i128, i128)> = [
        (47, 282),
        (70, 1079),
        (75, 1147),
        (66, 1062),
    ].into();

    for game in games {
        p1 *= ways(game.0, game.1);
    }
    
    let p2 = ways(47707566, 282107911471062);

    println!("#1 {}", p1);
    println!("#2 {}", p2);
}
