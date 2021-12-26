use crate::modules::ascii85::decode_ascii85;

/*

Like all the layers, the payload is again encoded with
Adobe-flavoured ASCII85. After ASCII85 decoding the payload,
apply the following operations to each byte:

  1. Flip every second bit
  2. Rotate the bits one position to the right

For example:

                        |      Binary      Decimal  Hex
  ----------------------+-------------------------------
  Starting value        | 1 0 1 1 0 1 0 0    180     B4
                        |   v   v   v   v
  Flip every second bit | 1 1 1 0 0 0 0 1    225     E1
                        |  \ \ \ \ \ \ \ \
  Rotate to the right   | 1 1 1 1 0 0 0 0 )  240     F0
                        |  \_____________/


*/
pub fn decode_layer0(orig: &String) -> String {
    let decoded = decode_ascii85(&orig);

    let mask = 0b01010101;

    let mut res = vec![];

    for b in decoded.iter() {
        let mut b = *b;

        // flip bits
        b = b ^ mask;

        // rotate
        let lb = b & 1;
        let b = b >> 1 + (lb << 7);

        // push
        res.push(b);
    }

    String::from_utf8(res).unwrap()
}
