use itertools::Itertools;

use crate::{
    crack_utils::{guess_keysizes, transpose, CrackedMessage},
    sorted::Sorted,
};

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

pub fn single(bytes: &[u8]) -> Vec<CrackedMessage<u8>> {
    (0x00..=0xFF)
        .filter_map(|b| {
            let message = fixed(bytes, &vec![b; bytes.len()]).unwrap();

            CrackedMessage::try_from((b, message)).ok()
        })
        .collect::<Vec<_>>()
        .sorted()
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
                .filter_map(|block| single(&block).first().map(|res| res.key))
                .collect();

            if key.len() != keysize {
                return None;
            }

            let decoded = repeating(input, &key);

            CrackedMessage::try_from((key, decoded)).ok()
        })
        .collect_vec()
        .sorted()
}
