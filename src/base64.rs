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
        // _ => None,
        // }
    }

    fn get_padding_char() -> char {
        '='
    }
}
