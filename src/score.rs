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
    b"abcdefghijklmnopqrstuvwxyz \n.,?!:'-@#$%^&*()[]{}\\/;`~_+=<>\"0123456789"
        .iter()
        .copied()
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

        let (char_counts, other_char_counts): (Vec<_>, Vec<_>) = bytes
            .iter()
            .map(|b| if *b == b'\r' { &b'\n' } else { b })
            .map(u8::to_ascii_lowercase)
            .dedup_with_count()
            .partition(|(_, byte)| ascii().contains(byte));

        let mut ascii_counts = ascii()
            .map(|b| {
                let char_count = char_counts
                    .iter()
                    .find_map(|(count, char)| (*char).eq(&b).then_some(*count))
                    .unwrap_or(0);

                ((char_count * 10000) / bytes.len(), b)
            })
            .collect_vec();

        // Add all the other char counts together and store them
        // represented by a single byte, '\0'
        let other_char_counts_combined = other_char_counts.into_iter().map(|t| t.0).sum();
        ascii_counts.push((other_char_counts_combined, b'\0'));

        Some(Self { ascii_counts })
    }
}

impl Score {
    fn diff(&self, other: &Self) -> usize {
        self.ascii_counts
            .iter()
            .map(|(count, byte)| {
                let (other_count, _) = other
                    .ascii_counts
                    .iter()
                    .find(|ac| ac.1.eq(byte))
                    .expect("Both vecs should contain the same ascii characters");

                count.abs_diff(*other_count)
            })
            .sum()
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

    #[test_case(SAMPLE_TEXT, 0; "Sample text")]
    #[test_case(REAL_TEXT, 2042; "Real text")]
    #[test_case(GIBBERISH, 5365; "Gibberish")]
    #[test_case(SOME_BYTES, 6803; "Some bytes")]
    fn score(input: &str, expected: usize) {
        assert_eq!(Some(expected), super::score(input.as_bytes()));
    }

    #[test]
    fn test_score_skips_inputs_with_unprintable_chars() {
        let input = b"js*^\x95\x113278";

        assert_eq!(None, super::score(input));
    }
}
