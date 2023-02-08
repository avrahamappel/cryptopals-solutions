use cryptopals::base64;
use cryptopals::hex;
use cryptopals::xor;

fn main() {
    let input = base64::decode(include_str!("../data/6.txt"));

    let possibilites = xor::repeating_crack(&input);

    println!();
    println!("Possibilities:");

    for p in possibilites {
        println!("Key: {}", hex::encode(&p.key));
        println!("Decoded: {}", String::from_utf8_lossy(&p.message));
        println!("Score: {}", p.score);
    }
}
