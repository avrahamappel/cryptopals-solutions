use cryptopals::hex;
use cryptopals::sorted::Sorted;
use cryptopals::xor;

fn main() {
    // Detect single-character XOR
    // One of the 60-character strings in this file has been encrypted by single-character XOR.

    // Find it.

    // (Your code from #3 should help.)
    let strings = include_str!("../data/4.txt");

    let results = strings
        .lines()
        .flat_map(|l| {
            println!("DECODING: {}", l);
            let bytes = hex::decode(l.trim());

            xor::single(&bytes)
        })
        .collect::<Vec<_>>()
        .sorted();

    for res in results.iter().take(10) {
        println!(
            "XOR by '{}' (probability: {}%): {}",
            char::from(res.byte),
            res.score,
            String::from_utf8_lossy(&res.message)
        );
    }
}
