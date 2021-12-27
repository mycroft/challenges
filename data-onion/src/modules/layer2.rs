use crate::modules::ascii85::decode_ascii85;

/*

Exclusive Or (XOR) is another bitwise operation. It's often
used in cryptography to combine two sources of binary data
-- for example, to combine binary data with a secret key,
resulting in scrambled output data.

What makes XOR useful, compared to other bitwise operations
such as AND or OR, is that it can be reversed without losing
any information. If you know the output and one of the
inputs, you can determine what the other input was. It
enables encryption algorithms to be undone, so that data can
be decrypted back to its original state.

For example, let's say we have two input bytes, A and B, and
the result of XOR'ing these two bytes together is another
byte C:

    A XOR B == C

If we have bytes C and B, we're able to determine what A was
by XOR'ing together C and B:

    C XOR B == A

Likewise, if we have bytes C and A, XOR'ing them together
will produce B:

    C XOR A == B

Using XOR by itself to encrypt data is very, very insecure,
as you're about to discover. Good encryption algorithms
still use XOR at certain points, but they have many steps
with various different data transformations.

*/
/*
Note to self:
- First used "==[ Layer 4/6: ?????????????????" & "??????????????===============\n\n??" to get most key parts
- Found out the first extracted string was: "==[ Layer 4/6: Network Traff^c}^Q============================="
- Changed first part, and disabled 2nd part detection.
 */
pub fn decode_layer2(orig: &str) -> String {
    let buf1 = "==[ Layer 4/6: ?????????????????"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let buf2 = "??????????????===============\n\n??"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let buf3 = "==[ Layer 4/6: Network Traffic ]"
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();

    let mut key = vec![0u8; 32];

    let decoded = decode_ascii85(orig);

    for idx in 0..32 {
        if buf1[idx] == b'?' {
            continue;
        }

        key[idx] = decoded[idx] ^ buf1[idx];
    }

    for idx in 0..32 {
        if buf2[idx] == b'?' {
            continue;
        }

        key[idx] = decoded[idx + 32] ^ buf2[idx];
    }

    // Added after first try to get final key:
    for idx in 0..32 {
        if buf3[idx] == b'?' {
            continue;
        }

        key[idx] = decoded[idx] ^ buf3[idx];
    }

    let mut idx = 0;
    let mut res = vec![];

    while decoded.len() > idx + 32 {
        for i in 0..32 {
            res.push(decoded[idx + i] ^ key[i]);
        }

        idx += 32;
    }

    String::from_utf8(res).unwrap()
}
