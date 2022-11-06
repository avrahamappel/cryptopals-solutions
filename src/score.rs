use std::collections::HashMap;
use std::ops::Sub;

const SAMPLE_TEXT: &str = r#"
    Single-byte cipher

    The hex encoded string has been XOR'd against a single character. Find the key, decrypt the message.

    You can do this by hand. But don't: write code to do it for you.

    How? Devise (no need to optimize) some method for "scoring" a piece of English plaintext. Character
    frequency is a good metric. Evaluate each output and choose the one with the best score.

    Achievement Unlocked
    You now have our permission to make jokes on Twitter."#;

fn round_to_2(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn ratio_to_percent(num: usize, den: usize) -> f64 {
    round_to_2((num * 100) as f64 / den as f64)
}

fn char_frequency(text: &[u8]) -> HashMap<u8, f64> {
    text.iter()
        .fold(HashMap::new(), |mut hash, byte| {
            let key = match byte {
                b'A'..=b'Z' | b'a'..=b'z' => byte.to_ascii_lowercase(),
                // 'P' for punctuation
                b',' | b'.' | b'!' | b'?' | b'\'' | b'"' | b'(' | b')' | b'-' => b'P',
                // 'S' for whitespace
                b' ' | b'\n' => b'S',
                // Other symbols
                _ => b'@',
            };

            hash.entry(key)
                .and_modify(|val| {
                    *val += 1;
                })
                .or_insert(1);
            hash
        })
        .into_iter()
        .map(|(byte, count)| {
            let percent = ratio_to_percent(count, text.len());
            (byte, percent)
        })
        .collect()
}

fn variances(bytes: &[u8]) -> Vec<f64> {
    // TODO cache this at compile time
    let base = char_frequency(SAMPLE_TEXT.as_bytes());
    let frequencies = char_frequency(bytes);

    frequencies
        .into_iter()
        .map(|(b, freq)| {
            let base_freq = base.get(&b).unwrap_or(&0.0);

            round_to_2(base_freq.sub(freq).abs())
        })
        .collect()
}

fn average(scores: Vec<f64>) -> f64 {
    round_to_2(scores.iter().sum::<f64>() / scores.len() as f64)
}

/// The score of a byte string. The lower the number, the more likely it is that the string is
/// English text.
pub fn score(bytes: &[u8]) -> f64 {
    average(variances(bytes))
}

#[cfg(test)]
mod tests {
    use super::SAMPLE_TEXT;

    const HELLO: &str = "Hello world!";
    const SOME_BYTES: &str = "6^*&^*^&%#";
    const GIBBERISH: &str = "eafaefjae swndajaqwkwfbvhb ydz";

    #[test]
    fn ratio_to_percent() {
        assert_eq!(20.0, super::ratio_to_percent(1, 5));
    }

    #[test]
    fn char_frequency() {
        for (input, expected) in [
            (
                SAMPLE_TEXT,
                &[
                    (b'a', 4.37),
                    (b'b', 1.04),
                    (b'c', 3.74),
                    (b'd', 3.33),
                    (b'e', 10.81),
                    (b'f', 1.04),
                    (b'g', 1.66),
                    (b'h', 4.16),
                    (b'i', 4.78),
                    (b'j', 0.21),
                    (b'k', 0.83),
                    (b'l', 1.25),
                    (b'm', 1.66),
                    (b'n', 4.78),
                    (b'o', 7.07),
                    (b'p', 1.46),
                    (b'q', 0.21),
                    (b'r', 3.74),
                    (b's', 3.95),
                    (b't', 6.86),
                    (b'u', 2.08),
                    (b'v', 0.83),
                    (b'w', 1.04),
                    (b'x', 0.62),
                    (b'y', 1.66),
                    (b'z', 0.21),
                    (b'P', 3.53),
                    (b'S', 22.87),
                    (b'@', 0.21),
                ][..],
            ),
            (
                HELLO,
                &[
                    (b'h', 8.33),
                    (b'e', 8.33),
                    (b'l', 25.0),
                    (b'o', 16.67),
                    (b'S', 8.33),
                    (b'w', 8.33),
                    (b'r', 8.33),
                    (b'd', 8.33),
                    (b'P', 8.33),
                ][..],
            ),
            (SOME_BYTES, &[(b'@', 100.0)][..]),
            (
                GIBBERISH,
                &[(b'e', 10.0), (b'a', 16.67), (b'f', 10.0), (b'j', 6.67)][..],
            ),
        ] {
            let frequencies = super::char_frequency(input.as_bytes());
            for (byte, frequency) in expected {
                assert_eq!((byte, frequency), frequencies.get_key_value(byte).unwrap());
            }
        }
    }

    #[test]
    fn score() {
        for (input, expected) in [
            (SAMPLE_TEXT, 0.0),
            (HELLO, 8.47),
            (GIBBERISH, 4.9),
            (SOME_BYTES, 99.79),
        ] {
            assert_eq!(expected, super::score(input.as_bytes()));
        }
    }
}
