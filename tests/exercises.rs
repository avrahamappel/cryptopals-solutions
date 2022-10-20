use cryptopals::base64;
use cryptopals::hex;
use cryptopals::xor;

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

#[test]
fn fixed_xor() {
    let first = hex::decode("1c0111001f010100061a024b53535009181c");

    let second = hex::decode("686974207468652062756c6c277320657965");
    assert_eq!("hit the bull's eye", String::from_utf8_lossy(&second));

    let res = xor::fixed(&first, &second).unwrap();
    assert_eq!("the kid don't play", String::from_utf8_lossy(&res));
    assert_eq!("746865206b696420646f6e277420706c6179", hex::encode(&res));
}
