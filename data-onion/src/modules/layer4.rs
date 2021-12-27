use crate::modules::ascii85::decode_ascii85;

/*
This payload has been encrypted with AES-256 in Counter Mode
(CTR). To decrypt the payload you will need the encryption
key and the initialization vector (IV). It is not possible
to guess these, so I will just give them to you. They are at
the start of the payload.

But... surprise! The key is also encrypted with AES. It
turns out that the U.S. Government also has standards for
how to encrypt encryption keys. I've encrypted the key using
the AES Key Wrap algorithm specified in RFC 3394. How do you
decrypt the key? Well, you will need another key, called the
"key encrypting key" (KEK), and another initialization
vector. These are also impossible to guess, so I will just
give them to you. They are also at the start of the payload.

But... surprise! Just kidding. I haven't encrypted the KEK.
The U.S. Government does not have a standard for encrypting
key encrypting keys, as far as I'm aware. That would be a
bit too crazy.

The payload is structured like this:

 - First 32 bytes: The 256-bit key encrypting key (KEK).
 - Next 8 bytes: The 64-bit initialization vector (IV) for
   the wrapped key.
 - Next 40 bytes: The wrapped (encrypted) key. When
   decrypted, this will become the 256-bit encryption key.
 - Next 16 bytes: The 128-bit initialization vector (IV) for
   the encrypted payload.
 - All remaining bytes: The encrypted payload.

The first step is to use the KEK and the 64-bit IV to unwrap
the wrapped key. The second step is to use the unwrapped key
and the 128-bit IV to decrypt the rest of the payload.

Don't try to write the decryption algorithms yourself. Or
do. I'm not your dad. You do you. Personally, I used OpenSSL
to generate the payload for this layer, and reused the
`aes_key_wrap` Ruby gem that I wrote years ago.
*/

use openssl::aes::{AesKey, unwrap_key};
use openssl::symm::{decrypt, Cipher};


pub fn decode_layer4(orig: &String) -> String {
    let decoded = decode_ascii85(&orig);

    let wrapped_key : Vec<u8> = decoded[40..80].into();
    let orig_iv : Vec<u8> = decoded[80..96].into();

    // First, unwrap key.

    let dec_key = AesKey::new_decrypt(decoded[0..32].into()).unwrap();
    let mut orig_key = [0u8; 32];

    unwrap_key(
        &dec_key,
        Some(decoded[32..40].try_into().expect("slice with correct length")), 
        &mut orig_key,
        &wrapped_key[..]).unwrap();

    // CTR AES with orig_key/orig_iv

    let out = decrypt(
        Cipher::aes_256_ctr(),
        &orig_key,
        Some(&orig_iv),
        &decoded[96..],
    ).unwrap();

    String::from_utf8(out).expect("valid utf8 text")
}