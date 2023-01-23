/// Return the Hamming distance of the bits in two byte strings.
pub fn distance(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter()
        .zip(s2)
        .map(|(b1, b2)| {
            [0x80, 0x40, 0x20, 0x10, 0x8, 0x4, 0x2, 0x1]
                .into_iter()
                .filter(|m| (b1 & m) != (b2 & m))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(37, distance(b"this is a test", b"wokka wokka!!!"));
    }
}
