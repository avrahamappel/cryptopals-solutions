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
    let base = char_frequency(SAMPLE_TEXT.as_bytes());
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
                    (b'a', 4.158004158004158),
                    (b'b', 0.8316008316008316),
                    (b'c', 3.5343035343035343),
                    (b'd', 3.1185031185031185),
                    (b'e', 10.395010395010395),
                    (b'f', 0.8316008316008316),
                    (b'g', 1.6632016632016633),
                    (b'h', 3.95010395010395),
                    (b'i', 4.781704781704781),
                    (b'j', 0.2079002079002079),
                    (b'k', 0.8316008316008316),
                    (b'l', 1.2474012474012475),
                    (b'm', 1.6632016632016633),
                    (b'n', 4.781704781704781),
                    (b'o', 6.860706860706861),
                    (b'p', 1.4553014553014554),
                    (b'q', 0.2079002079002079),
                    (b'r', 3.5343035343035343),
                    (b's', 3.7422037422037424),
                    (b't', 6.444906444906445),
                    (b'u', 1.8711018711018712),
                    (b'v', 0.8316008316008316),
                    (b'w', 1.0395010395010396),
                    (b'x', 0.4158004158004158),
                    (b'y', 1.2474012474012475),
                    (b'z', 0.2079002079002079),
                ][..],
            ),
            (
                HELLO,
                &[
                    (b'H', 8.333333333333334),
                    (b'e', 8.333333333333334),
                    (b'l', 25.0),
                    (b'o', 16.666666666666667),
                    (b' ', 8.333333333333334),
                    (b'w', 8.333333333333334),
                    (b'r', 8.333333333333334),
                    (b'd', 8.333333333333334),
                    (b'!', 8.333333333333334),
                ][..],
            ),
            (
                SOME_BYTES,
                &[
                    (b'6', 10.0),
                    (b'^', 30.0),
                    (b'*', 20.0),
                    (b'&', 20.0),
                    (b'%', 10.0),
                    (b'#', 10.0),
                ][..],
            ),
            (
                GIBBERISH,
                &[
                    (b'e', 10.0),
                    (b'a', 16.666666666666668),
                    (b'f', 10.0),
                    (b'j', 6.666666666666667),
                ][..],
            ),
        ] {
            let frequencies = super::char_frequency(input.as_bytes());
            for (byte, frequency) in expected {
                assert_eq!((byte, frequency), frequencies.get_key_value(byte).unwrap());
            }
        }
    }

    #[test]
    fn real_text_has_decent_score() {
        assert_eq!(5.0, super::score(b"Hello world!"));
    }

    #[test]
    fn sample_text_has_perfect_score() {
        assert_eq!(0.0, super::score(super::SAMPLE_TEXT.as_bytes()));
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
