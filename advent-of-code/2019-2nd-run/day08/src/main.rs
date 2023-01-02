use std::fs::read_to_string;

fn parse(fp: &str) -> Vec<isize> {
    let contents = read_to_string(fp).unwrap();

    contents.trim().chars().map(|x| x.to_digit(10).unwrap() as isize).collect::<Vec<isize>>()
}

fn get_layers(image: &Vec<isize>, width: usize, height: usize) -> Vec<Vec<isize>> {
    let mut result = Vec::new();
    let mut idx = 0;

    while idx + (width * height) <= image.len() {
        let layer: Vec<isize> = image[idx..idx+(width*height)].to_vec();

        result.push(layer.clone());
        idx += width * height;
    }

    result
}

fn find_step1_score(layers: &Vec<Vec<isize>>) -> usize {
    let mut number_of_zeroes = layers[0].len();
    let mut result = 0;


    for layer in layers {
        let mut numbers = [0usize; 3];

        for c in layer {
            numbers[*c as usize] += 1;
        }

        if number_of_zeroes > numbers[0] {
            number_of_zeroes = numbers[0];
            result = numbers[1] * numbers[2];
        }
    }

    result
}

fn decode_picture(layers: &Vec<Vec<isize>>) -> Vec<isize> {
    let mut result = Vec::new();

    for idx in 0..layers[0].len() {
        let mut found = false;
        for layer in layers {
            if layer[idx] != 2 {
                result.push(layer[idx]);
                found = true;
            }

            if found {
                break;
            }
        }
    }

    result
}

fn display(layer: &[isize], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if 1 == layer[x + y * width] {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!()
    }
    println!()
}

fn main() {
    let image = parse("input.txt");
    let layers = get_layers(&image, 25, 6);

    println!("#1 {}", find_step1_score(&layers));

    let decoded_image = decode_picture(&layers);
    println!("#2");
    display(&decoded_image, 25, 6);
}
