use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod modules;
use crate::modules::ascii85::*;
use crate::modules::layer0::*;

fn open_extract(fp: &str, filter: bool) -> String {
    let mut is_payload = !filter;

    let contents = fs::read_to_string(fp).expect("payload file");
    let lines  = contents
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {
            x.trim_end()
        })
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
    let payload = open_extract("payload", false);

    let layer_0 = decode_ascii85(&payload);
    if layer_0.is_err() {
        println!("Could not decode layer 0");
        return;
    }

    let layer_0 = layer_0.unwrap();

    let mut output = File::create("layer0").unwrap();
    write!(output, "{}", String::from_utf8(layer_0).unwrap()).unwrap();

    let payload = open_extract("layer0", true);


    let layer_1 = decode_layer0(&payload);

    let mut output = File::create("layer1").unwrap();
    write!(output, "{}", String::from_utf8(layer_1).unwrap()).unwrap();
}
