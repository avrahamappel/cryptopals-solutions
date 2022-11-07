use crate::score;

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

pub fn single(bytes: &[u8]) -> Vec<SingleByteXor> {
    let mut decoded = (0x00..=0xFF)
        .map(|b| {
            let message = fixed(bytes, &vec![b; bytes.len()]).unwrap();
            let score = score::score(&message);

            SingleByteXor {
                byte: b,
                message,
                score,
            }
        })
        .collect::<Vec<_>>();

    decoded.sort_by(|a, b| a.score.total_cmp(&b.score));

    decoded
}
