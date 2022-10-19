pub mod base64;
pub mod hex;
pub mod xor;

pub fn exercise() {
    /*
     * Write a function that takes two equal-length buffers and produces their XOR combination.

    If your function works properly, then when you feed it the string:

    1c0111001f010100061a024b53535009181c
    ... after hex decoding, and when XOR'd against:

    686974207468652062756c6c277320657965
    ... should produce:

    746865206b696420646f6e277420706c6179
    */
    let first = hex::decode("1c0111001f010100061a024b53535009181c");
    println!("first: {}", String::from_utf8_lossy(&first));
    let second = hex::decode("686974207468652062756c6c277320657965");
    println!("second: {}", String::from_utf8_lossy(&second));

    let res = xor::fixed(&first, &second).unwrap();
    println!("res: {}", String::from_utf8_lossy(&res));

    println!("{}", hex::encode(&res));
}
