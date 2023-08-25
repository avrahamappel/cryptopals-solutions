use cryptopals::base64;
use cryptopals::hex;
use cryptopals::sorted::Sorted;
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

#[test]
fn single_byte_xor() {
    let bytes = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let result = &xor::single(&bytes)[0];

    assert_eq!(b'X', result.key);
    assert_eq!(
        "Cooking MC's like a pound of bacon",
        String::from_utf8_lossy(&result.message)
    );
}

/// This test takes a long time when compiled in debug mode. I think the sorting
/// slows it down. If it starts slowing down in release mode, I'll look
/// into using `BTreeSets` or something instead of sorted Vecs.
#[test]
fn detect_single_char_xor() {
    let strings = include_str!("../data/4.txt");

    let results = strings
        .lines()
        .flat_map(|l| {
            println!("DECODING: {l}");
            let bytes = hex::decode(l.trim());

            xor::single(&bytes)
        })
        .collect::<Vec<_>>()
        .sorted();

    let result = &results[0];

    assert_eq!(b'5', result.key);
    assert_eq!(
        "Now that the party is jumping",
        String::from_utf8_lossy(&result.message).trim()
    );
}

#[test]
fn repeating_key_xor() {
    let input = "
Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal
"
    .trim()
    .as_bytes();

    let encrypted = xor::repeating(input, "ICE".as_bytes());

    assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f", hex::encode(&encrypted));
}
