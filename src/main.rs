use cryptopals::*;

fn main() {
    // AES in ECB mode

    // The Base64-encoded content in this file has been encrypted via AES-128 in ECB mode under the key
    let cipher = base64::decode(include_str!("../data/7.txt"));

    let key = "YELLOW SUBMARINE";
    // (case-sensitive, without the quotes; exactly 16 characters; I like "YELLOW SUBMARINE" because it's exactly 16 bytes long, and now you do too).

    // Decrypt it. You know the key, after all.
    let decoded = aes::decode(&cipher, key, 128);

    // println!("Decoded: {decoded}");
}
