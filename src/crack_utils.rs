use itertools::Itertools;

use crate::score;

pub enum CrackedMessageError {
    NoScore,
}

/// A cracked message possibility, with score
#[derive(Debug, Clone)]
pub struct CrackedMessage<K> {
    pub key: K,
    pub score: usize,
    pub message: Vec<u8>,
}

impl<K> TryFrom<(K, Vec<u8>)> for CrackedMessage<K> {
    type Error = CrackedMessageError;

    fn try_from((key, message): (K, Vec<u8>)) -> Result<Self, Self::Error> {
        score::score(&message)
            .map(|score| Self {
                key,
                score,
                message,
            })
            .ok_or(CrackedMessageError::NoScore)
    }
}

impl<K> PartialEq for CrackedMessage<K> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<K> Eq for CrackedMessage<K> {}

impl<K> Ord for CrackedMessage<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl<K> PartialOrd for CrackedMessage<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Transpose blocks of a byte string: construct a list of blocks where the first block is
/// comprised of the first byte of every block, the second is the second byte of every block, and
/// so on.
pub fn transpose(input: &[u8], size: usize) -> Vec<Vec<u8>> {
    input.chunks(size).fold(
        (0..size)
            .map(|_| Vec::with_capacity(input.len() / size))
            .collect(),
        |mut blocks: Vec<_>, chunk| {
            for (i, block) in blocks.iter_mut().enumerate() {
                if let Some(b) = chunk.get(i) {
                    block.push(*b);
                }
            }

            blocks
        },
    )
}

/// Return the Hamming distance of the bits in two byte strings.
fn hamming_distance(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter()
        .zip(s2)
        .map(|(b1, b2)| {
            [0x80, 0x40, 0x20, 0x10, 0x8, 0x4, 0x2, 0x1]
                .into_iter()
                .filter(|m| (b1 & m) != (b2 & m))
                .count()
        })
        .sum()
}

// Return a list of possible sizes of key used to encode the given bytes with
// repeating-key xor, ranked from most likely to least likely.
#[must_use]
pub fn guess_keysizes(input: &[u8], min: usize, max: usize) -> Vec<usize> {
    let max = max.min(input.len() / 2);

    let mut keysizes: Vec<_> = (min..=max)
        .map(|keysize| {
            let distances: Vec<_> = input
                .chunks(keysize)
                .take(4)
                .tuple_windows()
                .map(|(chunk1, chunk2)| hamming_distance(chunk1, chunk2))
                .collect();

            let average = distances.iter().sum::<usize>() / distances.len();
            let normalized = (average * 1000) / keysize;

            (normalized, keysize)
        })
        .collect();

    keysizes.sort_by(|(n1, _), (n2, _)| n1.cmp(n2));

    keysizes.into_iter().take(3).map(|(_, k)| k).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let input = b"123456789";

        assert_eq!(
            vec![
                vec![b'1', b'4', b'7'],
                vec![b'2', b'5', b'8'],
                vec![b'3', b'6', b'9'],
            ],
            transpose(input.as_slice(), 3)
        );
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(37, hamming_distance(b"this is a test", b"wokka wokka!!!"));
    }
}
