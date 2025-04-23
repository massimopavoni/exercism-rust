use std::iter::once;

use modinverse::modinverse;
use num_integer::gcd;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) == 1 {
        Ok(plaintext
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
                    (b'a'..=b'z') => {
                        Some(space.chain(once(((a * (x - 97) as i32 + b) % 26 + 97) as u8 as char)))
                    }
                    (b'0'..=b'9') => Some(space.chain(once(x as char))),
                    _ => None,
                }
            })
            .flatten()
            .collect())
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if let Some(a_modinv) = modinverse(a, 26) {
        println!("a_modinv: {}", a_modinv);
        Ok(ciphertext
            .bytes()
            .filter_map(|y| match y {
                (b'a'..=b'z') => {
                    Some(((a_modinv * (y as i32 - 97 - b)).rem_euclid(26) + 97) as u8 as char)
                }
                (b'0'..=b'9') => Some(y as char),
                _ => None,
            })
            .collect())
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}
