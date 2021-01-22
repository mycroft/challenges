pub fn start(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.".to_string(),
        x => format!("{} bottles of beer on the wall, {} bottles of beer.", x, x),
    }
}

pub fn end(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string(),
        1 => "Take it down and pass it around, no more bottles of beer on the wall.".to_string(),
        2 => "Take one down and pass it around, 1 bottle of beer on the wall.".to_string(),
        x => format!("Take one down and pass it around, {} bottles of beer on the wall.", x - 1),
    }
}

pub fn verse(n: u32) -> String {
    let mut res = String::new();

    res.push_str(start(n).as_str());
    res.push('\n');
    res.push_str(end(n).as_str());
    res.push('\n');

    res
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    let mut n = start + 1;

    while n != end {
        n -= 1;

        if !song.is_empty() {
            song.push('\n');
        }

        song.push_str(verse(n).as_str());
    }
    song
}
