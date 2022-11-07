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
            let bytes = hex::decode(l.trim());

            xor::single(&bytes)
                .into_iter()
                .map(|res| (l.to_owned(), res))
        })
        .collect::<Vec<_>>();

    results.sort_by(|a, b| a.1.score.total_cmp(&b.1.score));

    for (l, res) in results.iter().take(10) {
        println!("Hex: {}", l);
        println!(
            "Decode by '{}' (probability: {}%): {}",
            char::from(res.byte),
            res.score,
            String::from_utf8_lossy(&res.message)
        );
    }
}
