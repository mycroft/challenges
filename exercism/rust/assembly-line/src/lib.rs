// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success : f64 = match speed {
        0 => 0.,
        1..=4 => 100.,
        5..=8 => 90.,
        9..=10 => 77.,
        _ => unreachable!("This should not happen")
    };

    (221. * success) / 100. * speed as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.) as u32
}
