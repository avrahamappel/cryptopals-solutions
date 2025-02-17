use cryptopals::base64;

fn main() {
    let solution =
        &cryptopals::aes::aes_crack(&base64::decode(include_str!("../data/8.txt")), 2, 40)[0];

    println!("{solution:?}");
}
