mod base64;
mod hex;

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex::decode(hex);

    println!("String: {}", std::str::from_utf8(&bytes).unwrap());
    // Should produce:
    // SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
    println!("Base 64: {}", base64::encode(&bytes));
}
