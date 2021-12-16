fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");

    let res = convert(&contents);
    println!("#1 {}", res.0);
    println!("#2 {}", res.1);
}

fn bits_to_integer(bits: &[bool]) -> u128 {
    let mut res = 0;

    for i in 0..bits.len() {
        if bits[i] {
            res |= 1 << (bits.len() - i - 1);
        }
    }
    
    res
}


// First is version sum, Second is all operators resolved.
fn decode_packet(packet: &[bool], index: usize) -> (u128, u128, usize) {
    // three first bits are packet versions
    let packet_version: &[bool] = &packet[index..index+3];
    let type_id: &[bool] = &packet[index+3..index+6];
    let mut consumed = 0;
    let mut total_version = 0;

    let packet_version = bits_to_integer(packet_version);
    let type_id = bits_to_integer(type_id);

    total_version += packet_version;
    consumed += 6;

    if type_id == 4 {
        let mut number_bits = vec![];
        loop {
            let prefix = packet[index+consumed];
            consumed += 1;

            number_bits.append(&mut packet[index+consumed..index+consumed+4].to_vec());
            consumed += 4;

            if !prefix {
                break;
            }
        }

        let res = number_bits.iter().fold(0u128, |mut acc, &b| { acc <<= 1; acc |= b as u128; acc});

        (total_version, res, index + consumed)
    } else {
        let length_type_id = packet[index+consumed];
        consumed += 1;
        let length: &[bool] = if length_type_id {
            consumed += 11;
            &packet[index+consumed-11..index+consumed]
        } else {
            consumed += 15;
            &packet[index+consumed-15..index+consumed]
        };
        let length = bits_to_integer(length);
        let mut numbers = vec![];

        if !length_type_id {
            let mut sub_packet_consumed = 0;

            while sub_packet_consumed < length as usize {
                let orig_index = index + consumed;
                let res = decode_packet(packet, index + consumed);

                total_version += res.0;

                sub_packet_consumed += res.2 - orig_index;
                consumed += res.2 - orig_index;

                numbers.push(res.1);
            }    
        } else {
            for _ in 0..length {
                let orig_index = index + consumed;
                let res = decode_packet(packet, orig_index);

                total_version += res.0;
                consumed += res.2 - orig_index;
                numbers.push(res.1);
            }
        }

        let res = match type_id {
            0 => numbers.iter().sum(),
            1 => numbers.iter().product(),
            2 => *numbers.iter().min().unwrap(),
            3 => *numbers.iter().max().unwrap(),
            5 => (numbers[0] > numbers[1]) as u128,
            6 => (numbers[0] < numbers[1]) as u128,
            7 => (numbers[0] == numbers[1]) as u128,
            _ => unreachable!()
        };

        (total_version, res, index + consumed)
    }
}

// First is version sum
// Second is operator resolution.
fn convert(s: &str) -> (u128, u128) {
    let s = s.chars().collect::<Vec<char>>();

    let mut bits: Vec<bool> = vec![];

    for c in &s {
        let mut z = c.to_digit(16).expect("hexadecimal number");
        let mut current_bits = vec![];

        for _ in 0..4 {
            current_bits.insert(0, z % 2 == 1);
            z /= 2;
        }
        bits.append(&mut current_bits);
    }

    let res = decode_packet(&bits, 0);

    (res.0, res.1)
}

#[test]
fn test0() {
    assert_eq!(16, convert("8A004A801A8002F478").0);
    assert_eq!(12, convert("620080001611562C8802118E34").0);
    assert_eq!(23, convert("C0015000016115A2E0802F182340").0);
    assert_eq!(31, convert("A0016C880162017C3686B18A3D4780").0);
}

#[test]
fn test1() {
    assert_eq!(3, convert("C200B40A82").1);
    assert_eq!(54, convert("04005AC33890").1);
    assert_eq!(7, convert("880086C3E88112").1);
    assert_eq!(9, convert("CE00C43D881120").1);
    assert_eq!(1, convert("D8005AC2A8F0").1);
    assert_eq!(0, convert("F600BC2D8F").1);
    assert_eq!(0, convert("9C005AC2F8F0").1);
    assert_eq!(1, convert("9C0141080250320F1802104A08").1);
}