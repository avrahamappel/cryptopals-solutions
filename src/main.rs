use cryptopals::{hex, xor};

fn main() {
    // Detect single-character XOR
    // One of the 60-character strings in this file has been encrypted by single-character XOR.

    // Find it.

    // (Your code from #3 should help.)
    let strings = include_str!("../data/4.txt");

    let mut results = strings
        .lines()
        .flat_map(|l| {
            println!("DECODING: {}", l);
            let bytes = hex::decode(l.trim());

            xor::single(&bytes).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    results.sort_by(|a, b| a.score.to_bits().cmp(&b.score.to_bits()));

    for res in results.iter().take(10) {
        println!(
            "XOR by '{}' (probability: {}%): {}",
            char::from(res.byte),
            res.score,
            String::from_utf8_lossy(&res.message)
        );
    }
}
