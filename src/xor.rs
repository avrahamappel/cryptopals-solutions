use crate::hamming;
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

/// A single-byte XOR possiblity, with score
pub struct SingleByteXor {
    pub byte: u8,
    pub score: f64,
    pub message: Vec<u8>,
}

impl PartialEq for SingleByteXor {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for SingleByteXor {}

impl Ord for SingleByteXor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.to_bits().cmp(&other.score.to_bits())
    }
}

impl PartialOrd for SingleByteXor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn single(bytes: &[u8]) -> Vec<SingleByteXor> {
    (0x00..=0xFF)
        .map(|b| {
            let message = fixed(bytes, &vec![b; bytes.len()]).unwrap();
            let score = score::score(&message);

            SingleByteXor {
                byte: b,
                message,
                score,
            }
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
    let mut keysizes: Vec<_> = (2..=40)
        .map(|keysize| {
            let chunk1 = &input[..keysize];
            let chunk2 = &input[keysize..(keysize + 1)];

            let dist: f32 = hamming::distance(chunk1, chunk2) as f32;

            let normalized: f32 = dist / keysize as f32;

            (normalized, keysize)
        })
        .collect();

    keysizes.sort_by(|(n1, _), (n2, _)| n1.total_cmp(n2));

    keysizes.into_iter().map(|(_, k)| k).collect()
}

/// Crack repeating-key xor.
pub fn repeating_crack(input: &[u8]) {
    for keysize in guess_keysizes(input).iter() {
        println!("Keysize: {keysize}");

        let key: Vec<_> = transpose(input, *keysize)
            .into_iter()
            .map(|block| {
                // println!("Block: {}", String::from_utf8_lossy(&block));
                let res = &single(&block)[0];

                // println!("Possible key:");
                // println!("Byte: {}", res.byte);
                // println!("Decoded block: {}", String::from_utf8_lossy(&res.message));
                // println!("Probability: {}", res.score);

                res.byte
            })
            // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.
            .collect();

        println!("Key: {}", String::from_utf8_lossy(&key));

        let decoded = repeating(input, &key);

        println!("Decoded: {}", String::from_utf8_lossy(&decoded));
    }
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
    fn test_repeating_crack() {
        todo!()
    }
}
