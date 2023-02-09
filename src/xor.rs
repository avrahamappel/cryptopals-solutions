use itertools::Itertools;

use crate::hamming;
// use crate::hex;
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
    .unwrap()
}

pub enum CrackedMessageError {
    NoScore,
}

/// A cracked message possibility, with score
#[derive(Debug, Clone)]
pub struct CrackedMessage<K> {
    pub key: K,
    pub score: f64,
    pub message: Vec<u8>,
}

impl<K> TryFrom<(K, Vec<u8>)> for CrackedMessage<K> {
    type Error = CrackedMessageError;

    fn try_from((key, message): (K, Vec<u8>)) -> Result<Self, Self::Error> {
        score::score(&message)
            .map(|score| Self {
                key,
                message,
                score,
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
        self.score.to_bits().cmp(&other.score.to_bits())
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
    input.chunks_exact(size).fold(
        (0..size)
            .map(|_| Vec::with_capacity(input.len() / size))
            .collect(),
        |mut blocks: Vec<_>, chunk| {
            for i in 0..size {
                blocks[i].push(chunk[i]);
            }

            blocks
        },
    )
}

// Return a list of possible sizes of key used to encode the given bytes with
// repeating-key xor, ranked from most likely to least likely.
fn guess_keysizes(input: &[u8]) -> Vec<usize> {
    let max = 41.min(input.len() / 2);

    let mut keysizes: Vec<_> = (2..max)
        .map(|keysize| {
            let chunk1 = &input[..keysize];
            let chunk2 = &input[keysize..(keysize + 1)];

            let dist: f32 = hamming::distance(chunk1, chunk2) as f32;

            let normalized: f32 = dist / keysize as f32;

            (normalized, keysize)
        })
        .collect();

    keysizes.sort_by(|(n1, _), (n2, _)| n2.total_cmp(n1));

    for (n, k) in &keysizes {
        println!("N: {n}, K: {k}");
    }

    keysizes.into_iter().map(|(_, k)| k).collect()
}

/// Crack repeating-key xor.
pub fn repeating_crack(input: &[u8]) -> Vec<CrackedMessage<Vec<u8>>> {
    guess_keysizes(input)
        .into_iter()
        .take(3)
        .flat_map(|keysize| {
            // println!("Keysize: {keysize}");

            transpose(input, keysize)
                .into_iter()
                .map(|block| {
                    // println!("Block: {}", hex::encode(&block));

                    single(&block).first().map(|res| {
                        // println!("Possible key:");
                        // println!("Byte: {}", char::from(res.key));
                        // println!("Decoded block: {}", String::from_utf8_lossy(&res.message));
                        // println!("Probability: {}", res.score);

                        res.key
                    })
                })
                // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.
                .multi_cartesian_product()
                // .inspect(|x| {
                //     println!("Key chars: {}", x.iter().copied().map(char::from).join(""));
                // })
                .into_iter()
                .filter_map(|key| {
                    // println!("Key hex: {}", hex::encode(&key));

                    let decoded = repeating(input, &key);

                    // println!("Decoded: {}", String::from_utf8_lossy(&decoded));

                    CrackedMessage::try_from((key, decoded)).ok()
                })
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
    fn test_single() {
        let input = b"\x1A3\"q%v385$/&\"v\">?%w";
        let byte = b'V';

        assert_eq!(
            Some(b"Let's encrypt this!".to_vec()),
            super::fixed(input, &vec![byte; input.len()])
        );
    }

    #[test]
    fn test_crack_single() {
        let input = b"\x1A3\"q%v385$/&\"v\">?%w";

        assert_eq!(
            Some(&CrackedMessage {
                message: b"Let's encrypt this!".to_vec(),
                key: b'V',
                score: 1.37
            }),
            super::single(input).first()
        );
    }

    #[test]
    fn test_repeating() {
        assert_eq!(
            b"\x07\r\x07\x13H\x12\x0EO>\x0CN\x02\x01\x08N2\x07\x04\x04\x06A\x14E-\x1B\x1DC\x0E\x0B\x01oL\\".to_vec(),
            repeating(b"Four score and seven years ago...", b"Abraham Lincoln")
        );
    }

    #[test]
    fn test_crack_repeating() {
        assert_eq!(
            Some(&CrackedMessage {
                key: b"Abraham Lincoln".to_vec(),
                message: b"Four score and seven years ago...".to_vec(),
                score: 0.0
            }),
            repeating_crack(b"\x07\r\x07\x13H\x12\x0EO>\x0CN\x02\x01\x08N2\x07\x04\x04\x06A\x14E-\x1B\x1DC\x0E\x0B\x01oL\\".as_slice()).first()
        );
    }
}
