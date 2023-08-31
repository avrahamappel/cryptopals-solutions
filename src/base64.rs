const ALPHABET_TABLE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const PADDING_CHAR: char = '=';

fn byte2char(byte: u8) -> char {
    char::from(ALPHABET_TABLE[usize::from(byte)])
}

pub fn encode(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    bytes
        .chunks(3)
        .flat_map(
            |chunk| match (chunk[0], chunk.get(1).map(|b2| (b2, chunk.get(2)))) {
                (b1, Some((b2, Some(b3)))) => [
                    byte2char(b1 >> 2),
                    byte2char((b1 & 0b00000011) << 4 | b2 >> 4),
                    byte2char((b2 & 0b00001111) << 2 | b3 >> 6),
                    byte2char(b3 & 0b00111111),
                ],
                (b1, Some((b2, None))) => [
                    byte2char(b1 >> 2),
                    byte2char((b1 & 0b00000011) << 4 | b2 >> 4),
                    byte2char((b2 & 0b00001111) << 2),
                    PADDING_CHAR,
                ],
                (b1, None) => [
                    byte2char(b1 >> 2),
                    byte2char((b1 & 0b00000011) << 4),
                    PADDING_CHAR,
                    PADDING_CHAR,
                ],
            },
        )
        .collect()
}

pub fn decode(string: &str) -> Vec<u8> {
    if string.is_empty() {
        return Vec::new();
    }

    let encoded_bytes: Vec<_> = string
        .bytes()
        .filter_map(|b| {
            ALPHABET_TABLE
                .iter()
                .enumerate()
                .find_map(|(i, t)| (*t == b).then(|| i as u8))
        })
        .collect();

    encoded_bytes
        .chunks(4)
        .flat_map(|chunk| {
            match (
                chunk[0],
                chunk[1],
                chunk.get(2).map(|b3| (b3, chunk.get(3))),
            ) {
                (b1, b2, Some((b3, Some(b4)))) => vec![
                    // last 6 of b1 and second pair of b2
                    b1 << 2 | b2 >> 4,
                    // last 4 of b2 and 3rd to 6th of b3
                    b2 << 4 | b3 >> 2,
                    // last 2 of b3 and all of b4
                    b3 << 6 | b4,
                ],
                (b1, b2, Some((b3, None))) => vec![
                    // last 6 of b1 and second pair of b2
                    b1 << 2 | b2 >> 4,
                    // last 4 of b2 and 3rd to 6th of b3
                    b2 << 4 | b3 >> 2,
                ],
                (b1, b2, None) => vec![
                    // last 6 of b1 and second pair of b2
                    b1 << 2 | b2 >> 4,
                ],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode() {
        assert_eq!("UGFuY2FrZQ==", super::encode("Pancake".as_bytes()));
    }

    #[test]
    fn decode() {
        assert_eq!("Pancake".as_bytes(), &super::decode("UGFuY2FrZQ=="));
    }

    #[test]
    fn it_skips_irrelevant_characters_while_decoding() {
        assert_eq!("Pancake".as_bytes(), &super::decode("UGFuY2\nFrZQ=="))
    }

    #[test]
    fn it_doesnt_corrupt_bytes() {
        let all_bytes = (0x00..=0xFF).collect::<Vec<_>>();

        assert_eq!(&all_bytes, &super::decode(&super::encode(&all_bytes)));
    }
}
