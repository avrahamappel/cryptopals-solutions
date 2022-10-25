use std::collections::HashMap;

const SAMPLE_TEXT: &str = r#"
    Single-byte cipher

    The hex encoded string has been XOR'd against a single character. Find the key, decrypt the message.

    You can do this by hand. But don't: write code to do it for you.

    How? Devise some method for "scoring" a piece of English plaintext. Character
    frequency is a good metric. Evaluate each output and choose the one with the best score.

    Achievement Unlocked
    You now have our permission to make jokes on Twitter."#;

fn char_frequency(text: &str) -> HashMap<u8, f64> {
    text.bytes()
        .fold(HashMap::new(), |mut hash, byte| {
            hash.entry(byte)
                .and_modify(|val| {
                    *val += 1;
                })
                .or_insert(1);
            hash
        })
        .into_iter()
        .map(|(byte, count)| {
            let percent = (count * text.len()) as f64 / 100.0;
            (byte, percent)
        })
        .collect()
}
