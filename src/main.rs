use cryptopals::base64;
use cryptopals::xor;

fn main() {
    let input = base64::decode(include_str!("../data/6.txt"));

    xor::repeating_crack(&input);
}
