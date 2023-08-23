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
    static ref BASE_SCORE: Score = Score::maybe_from(SAMPLE_TEXT.as_bytes()).unwrap();
}

fn round_to_2(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Score {
    avg_word_length: f64,
    avg_vowel_count: f64,
    avg_consonant_count: f64,
    avg_punctuation_count: f64,
}

impl Score {
    fn maybe_from(bytes: &[u8]) -> Option<Self> {
        let unprintables = bytes
            .iter()
            .filter(|b| match b {
                // 127 is DEL
                127 => true,

                // 10 and 13 are LF and CR, respectively
                10 | 13 => false,

                // First 31 ASCII bytes are unprintable
                0..=31 => true,

                // C1
                0x80..=0x9F => true,

                _ => false,
            })
            .count();

        if unprintables > 0 {
            return None;
        }

        let (awl, (avc, (acc, pc))): (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))) = bytes
            .split(|b| *b == b' ')
            .map(|w| {
                let word_length = w.len();
                let lower = w.to_ascii_lowercase();
                let vowels = lower.iter().filter(|b| b"aeiou".contains(b)).count();
                let consonants = lower
                    .iter()
                    .filter(|b| b"bcdfghjklmnpqrstvwxyz".contains(b))
                    .count();
                let puncs = w.iter().filter(|b| b.is_ascii_punctuation()).count();

                (
                    word_length as f64,
                    (vowels as f64, (consonants as f64, puncs as f64)),
                )
            })
            .unzip();

        Some(Self {
            avg_word_length: average(&awl),
            avg_vowel_count: average(&avc),
            avg_consonant_count: average(&acc),
            avg_punctuation_count: average(&pc),
        })
    }
}

impl Score {
    fn diff(self, other: Self) -> Self {
        Self {
            avg_word_length: (self.avg_word_length - other.avg_word_length).abs(),
            avg_vowel_count: (self.avg_vowel_count - other.avg_vowel_count).abs(),
            avg_consonant_count: (self.avg_consonant_count - other.avg_consonant_count).abs(),
            avg_punctuation_count: (self.avg_punctuation_count - other.avg_punctuation_count),
        }
    }

    fn avg(self) -> f64 {
        average(
            [
                self.avg_word_length,
                self.avg_vowel_count,
                self.avg_consonant_count,
                self.avg_punctuation_count,
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
pub(crate) fn score(bytes: &[u8]) -> Option<f64> {
    Score::maybe_from(bytes)
        .map(|score| score.diff(*BASE_SCORE).avg())
        .map(round_to_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // From https://randomwordgenerator.com/sentence.php
    const REAL_TEXT: &str = "She felt that chill that makes the hairs on the back of your neck when he walked into the room.";
    const GIBBERISH: &str = "eafaefjae swndajaqwkwfbvhb ydz";
    const SOME_BYTES: &str = "6^*&^*^&%#";

    #[test_case(SAMPLE_TEXT => Score {
        avg_word_length: 3.82,
        avg_vowel_count: 1.4,
        avg_consonant_count: 2.13,
        avg_punctuation_count: 0.18,
    }; "Sample text")]
    #[test_case(REAL_TEXT => Score {
        avg_word_length: 3.8,
        avg_vowel_count: 1.3,
        avg_consonant_count: 2.45,
        avg_punctuation_count: 0.05,
    }; "Real text")]
    #[test_case(GIBBERISH => Score {
        avg_word_length: 9.33,
        avg_vowel_count: 2.67,
        avg_consonant_count: 6.67,
        avg_punctuation_count: 0.0,
    }; "Gibberish")]
    #[test_case(SOME_BYTES => Score {
        avg_word_length: 10.0,
        avg_vowel_count: 0.0,
        avg_consonant_count: 0.0,
        avg_punctuation_count: 9.0,
    }; "Some bytes")]
    fn base_score(input: &str) -> Score {
        Score::maybe_from(input.as_bytes()).unwrap()
    }

    #[test_case(SAMPLE_TEXT, 0.0; "Sample text")]
    #[test_case(REAL_TEXT, 0.08; "Real text")]
    #[test_case(GIBBERISH, 2.79; "Gibberish")]
    // Have to record this kind
    #[test_case(SOME_BYTES, 4.63; "Some bytes")]
    fn score(input: &str, expected: f64) {
        assert_eq!(Some(expected), super::score(input.as_bytes()));
    }

    #[test]
    fn test_score_skips_inputs_with_unprintable_chars() {
        let input = b"js*^\x15\x113278";

        assert_eq!(None, super::score(input));
    }
}
