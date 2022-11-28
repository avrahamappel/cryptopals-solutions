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
