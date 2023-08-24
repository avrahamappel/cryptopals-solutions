use itertools::Itertools;
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

fn ascii() -> impl Iterator<Item = u8> {
    (0x00u8..=0xFFu8).filter(u8::is_ascii)
}

#[derive(Clone, Debug, PartialEq)]
struct Score {
    ascii_counts: Vec<(usize, u8)>,
}

impl Score {
    fn maybe_from(bytes: &[u8]) -> Option<Self> {
        let unprintables = bytes.iter().filter(|b| !b.is_ascii()).count();

        if unprintables > 0 {
            return None;
        }

        let char_counts = bytes.iter().copied().dedup_with_count().collect_vec();

        let ascii_counts = ascii()
            .map(|b| {
                let char_count = char_counts
                    .iter()
                    .find_map(|(count, char)| (*char).eq(&b).then_some(*count))
                    .unwrap_or(0);

                // if * 100 isn't precise enough, we can add more digits
                // no need for floats
                ((char_count * 100) / bytes.len(), b)
            })
            .collect_vec();

        Some(Self { ascii_counts })
    }
}

impl Score {
    // TODO actualy we need to compare each one seprate and add the diffs
    fn diff(&self, other: &Self) -> usize {
        self.total().abs_diff(other.total())
    }

    fn total(&self) -> usize {
        self.ascii_counts.iter().map(|t| t.0).sum()
    }
}

/// The score of a byte string. The lower the number, the
/// more likely it is that the string is English text.
pub(crate) fn score(bytes: &[u8]) -> Option<usize> {
    Score::maybe_from(bytes).map(|score| score.diff(&BASE_SCORE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // From https://randomwordgenerator.com/sentence.php
    const REAL_TEXT: &str = "She felt that chill that makes the hairs on the back of your neck when he walked into the room.";
    const GIBBERISH: &str = "eafaefjae swndajaqwkwfbvhb ydz";
    const SOME_BYTES: &str = "6^*&^*^&%#";

    // #[test_case(SAMPLE_TEXT => Score {
    //     avg_whitespace_count: 3.82,
    //     avg_vowel_count: 1.4,
    //     avg_consonant_count: 2.13,
    //     avg_punctuation_count: 0.18,
    // }; "Sample text")]
    // #[test_case(REAL_TEXT => Score {
    //     avg_whitespace_count: 3.8,
    //     avg_vowel_count: 1.3,
    //     avg_consonant_count: 2.45,
    //     avg_punctuation_count: 0.05,
    // }; "Real text")]
    // #[test_case(GIBBERISH => Score {
    //     avg_whitespace_count: 9.33,
    //     avg_vowel_count: 2.67,
    //     avg_consonant_count: 6.67,
    //     avg_punctuation_count: 0.0,
    // }; "Gibberish")]
    // #[test_case(SOME_BYTES => Score {
    //     avg_whitespace_count: 10.0,
    //     avg_vowel_count: 0.0,
    //     avg_consonant_count: 0.0,
    //     avg_punctuation_count: 9.0,
    // }; "Some bytes")]
    // fn base_score(input: &str) -> Score {
    //     Score::maybe_from(input.as_bytes()).unwrap()
    // }

    #[test_case(SAMPLE_TEXT, 0; "Sample text")]
    #[test_case(REAL_TEXT, 22; "Real text")]
    #[test_case(GIBBERISH, 48; "Gibberish")]
    // Have to record this kind
    #[test_case(SOME_BYTES, 60; "Some bytes")]
    fn score(input: &str, expected: usize) {
        assert_eq!(Some(expected), super::score(input.as_bytes()));
    }

    #[test]
    fn test_score_skips_inputs_with_unprintable_chars() {
        let input = b"js*^\x95\x113278";

        assert_eq!(None, super::score(input));
    }
}
