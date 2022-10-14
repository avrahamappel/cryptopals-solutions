fn join(chunk: &[u8]) -> u8 {
    let value = |c| match c {
        b'A'..=b'Z' => c - b'A' + 10,
        b'a'..=b'z' => c - b'a' + 10,
        b'0'..=b'9' => c - b'0',
        _ => unimplemented!(),
    };
    let (first, second) = if let &[first, second, ..] = chunk {
        (first, second)
    } else {
        unimplemented!()
    };

    (value(first) << 4) | value(second)
}

pub fn decode(s: &str) -> Vec<u8> {
    s.as_bytes().chunks(2).map(join).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        assert_eq!(
            Ok("Hello world!"),
            std::str::from_utf8(&super::decode("48656c6c6f20776f726c6421"))
        );
    }
}
