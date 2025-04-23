#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&value| {
            let mut bytes = Vec::with_capacity(2);

            bytes.push((value & 0x7f) as u8);
            let mut value = value >> 7;

            while value > 0 {
                bytes.push(0x80 | (value & 0x7f) as u8);
                value >>= 7;
            }

            bytes.reverse();
            bytes
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.last().expect("Expected at least one byte") & 0x80 != 0 {
        return Err(Error::IncompleteNumber);
    }

    let mut numbers = Vec::with_capacity(bytes.len() / 2);
    let (mut number, mut shift) = (0, 0u8);

    for &byte in bytes {
        number = (number << 7) | (byte & 0x7f) as u32;
        shift += 7;

        if byte & 0x80 == 0 {
            numbers.push(number);
            number = 0;
            shift = 0;
        } else if shift > 28 {
            return Err(Error::Overflow);
        }
    }

    Ok(numbers)
}
