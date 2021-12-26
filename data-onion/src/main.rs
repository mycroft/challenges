use std::fs;
use std::fs::File;
use std::io::Write;

mod modules;
use crate::modules::ascii85::*;
use crate::modules::layer0::*;
use crate::modules::layer1::*;
use crate::modules::layer2::*;
use crate::modules::layer3::*;

fn file_contents(fp: &str) -> String {
    fs::read_to_string(fp).expect("payload file")
}

fn parse(contents: &String, filter: bool) -> String {
    let mut is_payload = !filter;

    let lines = contents
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim_end())
        .collect::<Vec<&str>>();

    let mut payload = vec![];

    for &l in &lines {
        if l.is_empty() {
            continue;
        }

        if is_payload {
            payload.push(l);
        }

        if l.contains("==[ Payload ]==") {
            is_payload = true;
        }
    }

    payload.join("")
}

fn main() {
    let initial_contents = file_contents("payload");
    let payload = parse(&initial_contents, false);

    let layer_0 = decode_ascii85(&payload);
    // No need to decode first layer.
    let layer_0 = String::from_utf8(layer_0).unwrap();

    let mut fd = File::create("layer0").unwrap();
    write!(fd, "{}", layer_0).unwrap();

    // println!("{}", String::from_utf8(layer_0).unwrap());

    // Decoding layer 1
    let payload = parse(&layer_0, true);
    let layer_1 = decode_layer0(&payload);

    let mut fd = File::create("layer1").unwrap();
    write!(fd, "{}", layer_1).unwrap();

    // println!("{}", layer_1);

    // Decoding layer 2
    let payload = parse(&layer_1, true);
    let layer_2 = decode_layer1(&payload);

    let mut fd = File::create("layer2").unwrap();
    write!(fd, "{}", layer_2).unwrap();

    // println!("{}", layer_2);

    // Decoding layer 3
    let payload = parse(&layer_2, true);
    let layer_3 = decode_layer2(&payload);

    let mut fd = File::create("layer3").unwrap();
    write!(fd, "{}", layer_3).unwrap();

    // println!("{}", layer_3);

    // Decoding layer 4
    let payload = parse(&layer_3, true);
    let layer_4 = decode_layer3(&payload);

    let mut fd = File::create("layer4").unwrap();
    write!(fd, "{}", layer_4).unwrap();

    println!("{}", layer_4);
}
