//! Implementing Base64 from scratch in Rust - DEV Community
//! https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1

const UPPERCASEOFFSET: u8 = 65;
const LOWERCASEOFFSET: u8 = 71;
const DIGITOFFSET: u8 = 4;

fn get_char_for_index(index: u8) -> Option<char> {
    let char = match index {
        0..=25 => index + UPPERCASEOFFSET,
        26..=51 => index + LOWERCASEOFFSET,
        52..=61 => index - DIGITOFFSET,
        62 => 43,
        63 => 47,
        _ => return None,
    };

    Some(char.into())
}

fn get_index_for_char(char: u8) -> Option<u8> {
    let index = match char {
        65..=90 => char - UPPERCASEOFFSET,  // A-Z
        97..=122 => char - LOWERCASEOFFSET, // a-z
        48..=57 => char + DIGITOFFSET,      // 0-9
        43 => 62,                           // +
        47 => 63,                           // /
        _ => return None,
    };

    Some(index)
}

const fn get_padding_char() -> u8 {
    b'='
}

fn split(chunk: &[u8]) -> Vec<u8> {
    match chunk.len() {
        1 => vec![&chunk[0] >> 2, (&chunk[0] & 0b00000011) << 4],

        2 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2,
        ],

        3 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
            (&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6,
            &chunk[2] & 0b00111111,
        ],

        _ => unreachable!(),
    }
}

fn stitch(bytes: &[u8]) -> Vec<u8> {
    let out = match bytes.len() {
        2 => vec![
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b00001111) << 4,
        ],

        3 => vec![
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b00001111) << 4 | bytes[2] >> 2,
            (bytes[2] & 0b00000011) << 6,
        ],

        4 => vec![
            (bytes[0] & 0b00111111) << 2 | bytes[1] >> 4,
            (bytes[1] & 0b00001111) << 4 | bytes[2] >> 2,
            (bytes[2] & 0b00000011) << 6 | bytes[3] & 0b00111111,
        ],

        _ => unreachable!(),
    };

    out.into_iter().filter(|&x| x > 0).collect()
}

pub fn encode(bytes: &[u8]) -> String {
    bytes
        .chunks(3)
        .flat_map(split)
        .filter_map(get_char_for_index)
        .collect()
}

pub fn decode(string: &str) -> Vec<u8> {
    string
        .as_bytes()
        .chunks_exact(4)
        .map(|chunk| {
            chunk
                .iter()
                .filter(|b| **b != get_padding_char())
                .filter_map(|b| get_index_for_char(*b))
                .collect::<Vec<_>>()
        })
        .flat_map(|chars| stitch(&chars))
        .collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    // The binary representation of "Hi" is `01001000 01101001 00100001`
    #[test_case("H", vec![0b00010010, 0b00000000]; "One char")]
    #[test_case("Hi", vec![0b00010010, 0b00000110, 0b00100100]; "Two chars")]
    #[test_case("Hi!", vec![0b00010010, 0b00000110, 0b00100100, 0b00100001]; "Three chars")]
    fn split(input: &str, expected: Vec<u8>) {
        assert_eq!(expected, super::split(input.as_bytes()));
    }

    #[test]
    fn encode() {
        assert_eq!("UGFuY2FrZQ", super::encode("Pancake".as_bytes()));
    }

    // The binary representation of "Hi" is `01001000 01101001 00100001`
    #[test_case(&[0b00010010, 0b00000000], b"H".to_vec(); "One char")]
    #[test_case(&[0b00010010, 0b00000110, 0b00100100], b"Hi".to_vec(); "Two chars")]
    #[test_case(&[0b00010010, 0b00000110, 0b00100100, 0b00100001], b"Hi!".to_vec(); "Three chars")]
    fn stitch(input: &[u8], expected: Vec<u8>) {
        assert_eq!(expected, super::stitch(input));
    }

    #[test]
    fn decode() {
        assert_eq!(
            "Pancake",
            super::decode("UGFuY2FrZQ==")
                .into_iter()
                .map(char::from)
                .collect::<String>()
        );
    }
}
