use cryptopals::base64;
use cryptopals::hex;

#[test]
fn base64_encode() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex::decode(hex);

    assert_eq!(
        "I'm killing your brain like a poisonous mushroom",
        String::from_utf8_lossy(&bytes)
    );
    assert_eq!(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        base64::encode(&bytes)
    );
}
