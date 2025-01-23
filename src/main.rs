use cryptopals::base64;
use cryptopals::xor;

fn main() {
    let input = base64::decode(&include_str!("../data/6.txt").replace('\n', ""));

    let possibilities = xor::repeating_crack(&input, 2, 40);

    println!();
    println!("Possibilities:");

    for (i, p) in possibilities.iter().enumerate() {
        println!("{}.", i + 1);
        println!(
            "Key: (len {}) {}",
            p.key.len(),
            String::from_utf8_lossy(&p.key)
        );
        println!("Decoded: {}", String::from_utf8_lossy(&p.message));
        println!("Score: {}", p.score);
    }
}
