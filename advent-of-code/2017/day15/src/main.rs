fn main() {
    let factor_a = 16807;
    let factor_b = 48271;

    let divider = 2147483647;

    let mut val_a : u128 = 883;     // 65
    let mut val_b : u128 = 879;     // 8921

    let mut count = 0;

    for _i in 0..40_000_000 {
        val_a = (val_a * factor_a) % divider;
        val_b = (val_b * factor_b) % divider;

        if val_a & 0xffff == val_b & 0xffff {
            count += 1;
        }
    }

    println!("Part #1: {:?}", count);


    let mut val_a : u128 = 883;     // 65
    let mut val_b : u128 = 879;     // 8921

    let mut count = 0;

    for _i in 0..5_000_000 {
        loop {
            val_a = (val_a * factor_a) % divider;
            if val_a % 4 == 0 {
                break;
            }    
        }

        loop {
            val_b = (val_b * factor_b) % divider;
            if val_b % 8 == 0 {
                break;
            }
        }

        if val_a & 0xffff == val_b & 0xffff {
            count += 1
        }
    }

    println!("Part #2: {:?}", count);
}
