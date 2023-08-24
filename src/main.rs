use cryptopals::base64;
use cryptopals::xor;

fn main() {
    let input = base64::decode(&include_str!("../data/6-test.txt").replace('\n', ""));

    let possibilities = xor::repeating_crack(&input, 2, 20);

    println!();
    println!("Possibilities:");

    for p in possibilities {
        println!(
            "Key: (len {}) {}",
            p.key.len(),
            String::from_utf8_lossy(&p.key)
        );
        println!("Decoded: {}", String::from_utf8_lossy(&p.message));
        println!("Score: {}", p.score);
    }
}
// TODO test by encoding a known text of several lines with a known key
// The problem might be that we need to check potential solutions for being pure ascii
