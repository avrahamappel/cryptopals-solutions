//! Implementing Base64 from scratch in Rust - DEV Community
//! https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1

const UPPERCASEOFFSET: u8 = 65;
const LOWERCASEOFFSET: u8 = 71;
const DIGITOFFSET: u8 = 4;

struct Alphabet;

impl Alphabet {
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

    fn get_index_for_char(char: char) -> Option<u8> {
        if let Ok(char) = char.try_into() {
            let index = match char {
                65..=90 => char - UPPERCASEOFFSET,  // A-Z
                97..=122 => char - LOWERCASEOFFSET, // a-z
                48..=57 => char + DIGITOFFSET,      // 0-9
                43 => 62,                           // +
                47 => 63,                           // /
                _ => return None,
            };

            return Some(index);
        }

        None
    }

    fn get_padding_char() -> char {
        '='
    }
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

pub fn encode(bytes: &[u8]) -> String {
    bytes
        .chunks(3)
        .flat_map(split)
        .filter_map(Alphabet::get_char_for_index)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn split() {
        for (input, expected) in [
            // The binary representation of "Hi" is `01001000 01101001 00100001`
            ("H", vec![0b00010010, 0b00000000]),
            ("Hi", vec![0b00010010, 0b00000110, 0b00100100]),
            ("Hi!", vec![0b00010010, 0b00000110, 0b00100100, 0b00100001]),
        ] {
            assert_eq!(expected, super::split(input.as_bytes()));
        }
    }

    #[test]
    fn encode() {
        assert_eq!("UGFuY2FrZQ", super::encode("Pancake".as_bytes()));
    }
}
