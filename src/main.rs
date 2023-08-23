use cryptopals::base64;
use cryptopals::hex;
use cryptopals::xor;

fn main() {
    let input = base64::decode(include_str!("../data/6.txt"));

    let possibilities = xor::repeating_crack(&input, 528, 1024);

    println!("Possibilities:");

    for p in possibilities {
        println!("Key: (len {}) {}", p.key.len(), hex::encode(&p.key));
        println!("Decoded: {}", String::from_utf8_lossy(&p.message));
        println!("Score: {}", p.score);
    }
}
