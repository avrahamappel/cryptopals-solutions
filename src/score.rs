use std::collections::HashMap;
use std::ops::Sub;

const SAMPLE_TEXT: &[u8; 459] = br#"
    Single-byte cipher

    The hex encoded string has been XOR'd against a single character. Find the key, decrypt the message.

    You can do this by hand. But don't: write code to do it for you.

    How? Devise some method for "scoring" a piece of English plaintext. Character
    frequency is a good metric. Evaluate each output and choose the one with the best score.

    Achievement Unlocked
    You now have our permission to make jokes on Twitter."#;

fn ratio_to_percent(num: usize, den: usize) -> f64 {
    (num * 100) as f64 / den as f64
}

fn char_frequency(text: &[u8]) -> HashMap<u8, f64> {
    text.iter()
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
            let percent = ratio_to_percent(count, text.len());
            (*byte, percent)
        })
        .collect()
}

fn variances(bytes: &[u8]) -> Vec<f64> {
    // TODO cache this at compile time
    let base = char_frequency(SAMPLE_TEXT);
    let frequencies = char_frequency(bytes);

    frequencies
        .into_iter()
        .map(|(b, freq)| {
            let base_freq = base.get(&b).unwrap_or(&0.0);

            base_freq.sub(freq).abs()
        })
        .collect()
}

fn average(scores: Vec<f64>) -> f64 {
    scores.iter().sum::<f64>() / scores.len() as f64
}

/// The score of a byte string. The lower the number, the more likely it is that the string is
/// English text.
pub fn score(bytes: &[u8]) -> f64 {
    average(variances(bytes))
}

#[cfg(test)]
mod tests {
    #[test]
    fn ratio_to_percent() {
        assert_eq!(20.0, super::ratio_to_percent(1, 5));
    }

    #[test]
    fn real_text_has_decent_score() {
        assert_eq!(5.0, super::score(b"Hello world!"));
    }

    #[test]
    fn sample_text_has_perfect_score() {
        assert_eq!(0.0, super::score(super::SAMPLE_TEXT));
    }

    #[test]
    fn non_alphanumeric_gibberish_has_terrible_score() {
        assert_eq!(100.0, super::score(b"@%##%#@^^&%$"));
    }

    #[test]
    fn alphanumeric_gibberish_has_mediocre_score() {
        assert_eq!(50.0, super::score(b"djb2iu3h2hfffnc"));
    }
}
