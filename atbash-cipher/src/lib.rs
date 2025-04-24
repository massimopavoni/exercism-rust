use std::iter::once;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .bytes()
        .filter(u8::is_ascii_alphanumeric)
        .enumerate()
        .filter_map(|(i, x)| {
            let space = if i != 0 && i % 5 == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter();

            match x {
                (b'a'..=b'z') => Some(space.chain(once((b'z' - (x - b'a')) as char))),
                (b'0'..=b'9') => Some(space.chain(once(x as char))),
                _ => None,
            }
        })
        .flatten()
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .bytes()
        .filter_map(|y| match y {
            (b'a'..=b'z') => Some((b'z' - (y - b'a')) as char),
            (b'0'..=b'9') => Some(y as char),
            _ => None,
        })
        .collect()
}
