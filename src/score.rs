use integer_hasher::IntMap as HashMap;

use lazy_static::lazy_static;

// This is the text of the challenge
// (I added a tiny bit so I could get a 'z' in there)
const SAMPLE_TEXT: &str = r#"
    Single-byte cipher

    The hex encoded string has been XOR'd against a single character. Find the key, decrypt the message.

    You can do this by hand. But don't: write code to do it for you.

    How? Devise (no need to optimize) some method for "scoring" a piece of English plaintext. Character
    frequency is a good metric. Evaluate each output and choose the one with the best score.

    Achievement Unlocked
    You now have our permission to make jokes on Twitter."#;

lazy_static! {
    static ref BASE_FREQUENCIES: HashMap<u8, f64> = char_frequency(SAMPLE_TEXT.as_bytes());
    static ref BASE_SCORES: Score = Score::from(SAMPLE_TEXT.as_bytes());
}

fn round_to_2(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn ratio_to_percent(num: usize, den: usize) -> f64 {
    round_to_2((num * 100) as f64 / den as f64)
}

fn char_frequency(text: &[u8]) -> HashMap<u8, f64> {
    text.iter()
        .fold(HashMap::default(), |mut hash, byte| {
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

#[derive(Clone, Copy)]
struct Score {
    avg_word_length: f64,
    avg_vowel_count: f64,
    avg_consonant_count: f64,
}

impl From<&[u8]> for Score {
    fn from(bytes: &[u8]) -> Self {
        let (awl, (avc, acc)): (Vec<_>, (Vec<_>, Vec<_>)) = bytes
            .split(|b| *b == b' ')
            .map(|w| {
                let word_length = w.len();
                let lower = w.to_ascii_lowercase();
                let vowels = lower.iter().filter(|b| b"aeiou".contains(b)).count();
                let consonants = lower
                    .iter()
                    .filter(|b| b"bcdfghjklmnpqrstvwxyz".contains(b))
                    .count();
                (word_length as f64, (vowels as f64, consonants as f64))
            })
            .unzip();

        Self {
            avg_word_length: average(&awl),
            avg_vowel_count: average(&avc),
            avg_consonant_count: average(&acc),
        }
    }
}

impl Score {
    fn diff(self, other: Self) -> Self {
        Self {
            avg_word_length: (self.avg_word_length - other.avg_word_length).abs(),
            avg_vowel_count: (self.avg_vowel_count - other.avg_vowel_count).abs(),
            avg_consonant_count: (self.avg_consonant_count - other.avg_consonant_count).abs(),
        }
    }

    fn avg(self) -> f64 {
        average(
            [
                self.avg_word_length,
                self.avg_vowel_count,
                self.avg_consonant_count,
            ]
            .as_slice(),
        )
    }
}

fn average(values: &[f64]) -> f64 {
    round_to_2(values.iter().sum::<f64>() / values.len() as f64)
}

/// The score of a byte string. The lower the number, the
/// more likely it is that the string is English text.
pub fn score(bytes: &[u8]) -> f64 {
    round_to_2(Score::from(bytes).diff(*BASE_SCORES).avg())
}

#[cfg(test)]
mod tests {
    use super::SAMPLE_TEXT;

    // From https://randomwordgenerator.com/sentence.php
    const REAL_TEXT: &str = "She felt that chill that makes the hairs on the back of your neck when he walked into the room.";
    const GIBBERISH: &str = "eafaefjae swndajaqwkwfbvhb ydz";
    const SOME_BYTES: &str = "6^*&^*^&%#";

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
                    (b'S', 22.87),
                    (b'e', 10.81),
                    (b'o', 7.07),
                    (b't', 6.86),
                    (b'i', 4.78),
                    (b'n', 4.78),
                    (b'a', 4.37),
                    (b'h', 4.16),
                    (b's', 3.95),
                    (b'c', 3.74),
                    (b'r', 3.74),
                    (b'P', 3.53),
                    (b'd', 3.33),
                    (b'u', 2.08),
                    (b'g', 1.66),
                    (b'm', 1.66),
                    (b'y', 1.66),
                    (b'p', 1.46),
                    (b'l', 1.25),
                    (b'b', 1.04),
                    (b'f', 1.04),
                    (b'w', 1.04),
                    (b'k', 0.83),
                    (b'v', 0.83),
                    (b'x', 0.62),
                    (b'j', 0.21),
                    (b'q', 0.21),
                    (b'z', 0.21),
                    (b'@', 0.21),
                ][..],
            ),
            (
                REAL_TEXT,
                &[
                    (b'S', 20.00),
                    (b'h', 10.53),
                    (b'e', 10.53),
                    (b'o', 6.32),
                    (b'l', 4.21),
                    (b'r', 3.16),
                    (b'w', 2.11),
                    (b'd', 1.05),
                    (b'P', 1.05),
                ][..],
            ),
            (
                GIBBERISH,
                &[(b'a', 16.67), (b'e', 10.0), (b'f', 10.0), (b'j', 6.67)][..],
            ),
            (SOME_BYTES, &[(b'@', 100.0)][..]),
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
            (REAL_TEXT, 0.15),
            (GIBBERISH, 3.77),
            // Have to record this kind
            (SOME_BYTES, 3.24),
        ] {
            assert_eq!(expected, super::score(input.as_bytes()));
        }
    }
}
