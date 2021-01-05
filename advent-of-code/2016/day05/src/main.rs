use hex;

fn main() {
    let _door_id = "abbhdwsy";
    let mut id = 0;
    let mut password = String::new();
    let mut password2 = vec![0; 8];

    loop {
        let digest = md5::compute(format!("{}{}", _door_id, id));
        if hex::encode(&*digest).starts_with("00000") {
            let digest_str = format!("{:x}", digest);

            let password_char = digest_str.chars().nth(5).unwrap();
            let password_char_2 = digest_str.chars().nth(6).unwrap();

            // 1st password
            if password.len() < 8 {
                password.push(password_char);
            }

            // 2nd password
            if password_char >= '0' && password_char <= '7' {
                if password2[password_char.to_digit(10).unwrap() as usize] == 0 {
                    password2[password_char.to_digit(10).unwrap() as usize] = password_char_2 as u8;
                }
            }

            if !password2.iter().any(|x| *x == 0) {
                break;
            }
        }

        id += 1;
    }

    println!("Part #1: {}", password);
    println!("Part #2: {}", String::from_utf8(password2).unwrap());
}
