#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut n = 0;
    for &d in number {
        if d >= from_base {
            return Err(Error::InvalidDigit(d));
        }

        n = n * from_base + d;
    }

    if n == 0 {
        return Ok(vec![0]);
    }

    let mut m = vec![];
    while n > 0 {
        m.push(n % to_base);
        n /= to_base;
    }

    m.reverse();
    Ok(m)
}
