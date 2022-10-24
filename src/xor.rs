use std::collections::HashMap;

pub fn fixed(first: &[u8], second: &[u8]) -> Option<Vec<u8>> {
    if first.len() != second.len() {
        return None;
    }

    let xord = first
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ second[i])
        .collect();

    Some(xord)
}

const ETAOIN: &[u8; 95] = b" etaniosrldhc\numpfg.ybw,v0k1STC285A9x3I-647MB\"'PENFRDUqLGJHOWjz/<>K)(VY:QZX;?\x7F^&+[]$!*=~_\t{@\x05\x1B\x1E";

/// A single-byte XOR possiblity, with score
pub struct SingleByteXor {
    pub byte: u8,
    pub score: f64,
    pub message: Vec<u8>,
}

/// How many times does each ETAOIN byte occur in this string
fn occurences(bytes: &[u8]) -> Vec<(u8, usize)> {
    let mut occurences = ETAOIN
        .iter()
        .map(|b| {
            let occurences = bytes.iter().filter(|b1| **b1 == *b).count();

            (*b, occurences)
        })
        .collect::<Vec<_>>();

    occurences.sort_by(|a, b| b.1.cmp(&a.1));

    occurences
}

/// How much does the occurence of each ETAOIN byte deviate from most English text
fn occurence_weights(bytes: &[u8]) -> HashMap<u8, usize> {
    occurences(bytes)
        .into_iter()
        .enumerate()
        .map(|(i, (b, _))| (b, i))
        .collect()
}

/// Variance of occurences from ETAOIN value
fn variances(bytes: &[u8]) -> Vec<usize> {
    ETAOIN
        .iter()
        .enumerate()
        .map(|(i, b)| occurence_weights(bytes).get(b).unwrap().abs_diff(i))
        .collect()
}

fn average(scores: Vec<usize>) -> usize {
    scores.iter().sum::<usize>() / scores.len()
}

fn percent(number: usize) -> f64 {
    // (256 * 256 * 100) as f64 /
    number as f64
}

fn score(bytes: &[u8]) -> f64 {
    let percent = percent(average(variances(bytes)));
    percent

    //     if percent > 100.0 {
    //         0.0
    //     } else {
    //         100.0 - percent
    //     }
}

pub fn single(bytes: &[u8]) -> impl Iterator<Item = SingleByteXor> + '_ {
    (0x00..=0xFF).map(|b| {
        let message = fixed(bytes, &vec![b; bytes.len()]).unwrap();
        let score = score(&message);

        SingleByteXor {
            byte: b,
            message,
            score,
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn real_text_has_decent_score() {
        assert_eq!(75.0, super::score(b"Hello world!"));
    }

    #[test]
    fn uppercase_etoain_has_perfect_score() {
        assert_eq!(100.0, super::score(b"ETOAINSHRDLU"));
    }

    #[test]
    fn lowercase_etoain_has_perfect_score() {
        assert_eq!(100.0, super::score(b"etoainshrdlu"));
    }

    #[test]
    fn non_alphanumeric_gibberish_has_terrible_score() {
        assert_eq!(0.0, super::score(b"@%##%#@^^&%$"));
    }

    #[test]
    fn alphanumeric_gibberish_has_mediocre_score() {
        assert_eq!(50.0, super::score(b"djb2iu3h2hfffnc"));
    }
}
