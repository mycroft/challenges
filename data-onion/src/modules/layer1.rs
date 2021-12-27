use crate::modules::ascii85::decode_ascii85;

/*

For each byte of the payload, the seven most significant
bits carry data, and the least significant bit is the parity
bit. Combine the seven data bits from each byte where the
parity bit is correct, discarding bytes where the parity bit
is incorrect.

To determine if the parity bit is correct, first count how
many '1' bits exist within the seven data bits. If the count
is odd, the parity bit should be '1'. If the count is even,
the parity bit should be '0'.

For example, here is the byte 0xA3 (163 in decimal):

  1 0 1 0 0 0 1 1 <-- Parity bit (least significant bit)
  ^ ^ ^ ^ ^ ^ ^
  | | | | | | |
    Data bits

Of the data bits above, three of them are '1's. This is an
odd number, so the '1' parity bit is correct.

To make this layer a little bit easier, the byte size of the
payload is guaranteed to be a multiple of eight. Every group
of eight bytes contains 64 bits total, including 8 parity
bits. Removing the 8 parity bits leaves behind 56 data
bits, which is exactly 7 bytes.

*/
pub fn decode_layer1(orig: &str) -> String {
    let mut res = vec![];
    let mut bits = vec![];

    let decoded = decode_ascii85(orig);

    for mut c in decoded {
        let control_is_odd = c & 1 == 1; // is odd
        let mut c_bits = vec![];
        //println!("{:b}", c);
        c >>= 1;
        //println!("{:b} {}", c, control_is_odd);
        let mut is_even = true;

        for _ in 0..7 {
            let bit = if c & 1 == 1 {
                is_even = !is_even;
                true
            } else {
                false
            };

            c_bits.insert(0, bit);
            c >>= 1;
        }

        if is_even != control_is_odd {
            bits.append(&mut c_bits);
        }
    }

    // Rebuild characters from bits to res.
    let mut idx = 0;
    while bits.len() - idx >= 8 {
        let c = bits[idx..idx + 8]
            .iter()
            .fold(0, |acc, &b| (acc << 1) + b as u8) as u8;
        res.push(c);

        idx += 8;
    }

    String::from_utf8(res).unwrap()
}
