use itertools::Itertools;

use crate::score;
use crate::sorted::Sorted;

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

pub fn repeating(value: &[u8], key: &[u8]) -> Vec<u8> {
    fixed(
        value,
        key.iter()
            .copied()
            .cycle()
            .take(value.len())
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .expect("Key was empty")
}

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

pub fn single(bytes: &[u8]) -> Vec<CrackedMessage<u8>> {
    (0x00..=0xFF)
        .filter_map(|b| {
            let message = fixed(bytes, &vec![b; bytes.len()]).unwrap();

            CrackedMessage::try_from((b, message)).ok()
        })
        .collect::<Vec<_>>()
        .sorted()
}

/// Transpose blocks of a byte string: construct a list of blocks where the first block is
/// comprised of the first byte of every block, the second is the second byte of every block, and
/// so on.
fn transpose(input: &[u8], size: usize) -> Vec<Vec<u8>> {
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
                .take(6)
                .tuple_windows()
                .map(|(chunk1, chunk2)| {
                    let dist = hamming_distance(chunk1, chunk2) as f32;

                    dist / keysize as f32
                })
                .collect();

            let normalized = distances.iter().product::<f32>() / distances.len() as f32;

            (normalized, keysize)
        })
        .collect();

    keysizes.sort_by(|(n1, _), (n2, _)| n2.total_cmp(n1));

    keysizes.into_iter().map(|(_, k)| k).take(3).collect()
}

/// Crack repeating-key xor.
#[must_use]
pub fn repeating_crack(input: &[u8], min: usize, max: usize) -> Vec<CrackedMessage<Vec<u8>>> {
    guess_keysizes(input, min, max)
        .into_iter()
        .filter_map(|keysize| {
            let key: Vec<_> = transpose(input, keysize)
                .into_iter()
                // For each block, the single-byte XOR key that produces the best looking
                // histogram is the repeating-key XOR key byte for that block. Put them
                // together and you have the key.
                .filter_map(|block| {
                    let results = single(&block);

                    for res in &results[..] {
                        eprintln!(
                            "keysize: {keysize}, key: {}, score: {}, message: {}",
                            char::from(res.key),
                            res.score,
                            String::from_utf8_lossy(&res.message)
                        );
                    }

                    results.first().map(|res| res.key)
                })
                .collect();

            // eprintln!("KEYSIZE: {keysize}",);
            eprint!(".");

            if key.len() != keysize {
                return None;
            }

            let decoded = repeating(input, &key);

            CrackedMessage::try_from((key, decoded)).ok()
        })
        .collect_vec()
        .sorted()
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
