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

const ETOAIN: &[u8; 12] = b"ETAOINSHRDLU";

fn score(bytes: &[u8]) -> u32 {
    let mut occurences = ETOAIN
        .iter()
        .map(|b| {
            // occurences of b
            let occurences = bytes.iter().filter(|b1| *b1 == b).count();

            (b, occurences)
        })
        .collect::<Vec<_>>();

    occurences.sort_by(|a, b| b.1.cmp(&a.1));

    let occurences = occurences
        .into_iter()
        .enumerate()
        .map(|(i, (b, _))| (b, i))
        .collect::<HashMap<_, _>>();

    // variance of occurences from ETOAIN value
    let diff = |(i, b)| occurences.get(b).unwrap().abs_diff(i);
    let variances = ETOAIN
        .iter()
        .rev()
        .enumerate()
        .map(diff)
        .collect::<Vec<_>>();

    // average of variances
    (variances.iter().sum::<usize>() / variances.len()) as u32
}

pub fn single(bytes: &[u8]) -> impl Iterator<Item = (u8, Vec<u8>, u32)> + '_ {
    (0x00..=0xFF).map(|b| {
        let xord = fixed(bytes, &vec![b; bytes.len()]).unwrap();
        let score = score(&xord);
        (b, xord, score)
    })
}
