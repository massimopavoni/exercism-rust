#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        Err(Error::SpanTooLong)
    } else {
        if string_digits.is_empty() || span == 0 {
            return Ok(1);
        }

        let mut max_product = 0;

        for chunk in string_digits.as_bytes().windows(span) {
            let mut product = 1;

            for &digit in chunk {
                if !digit.is_ascii_digit() {
                    return Err(Error::InvalidDigit(digit as char));
                }

                product *= (digit - b'0') as u64;
            }

            if product > max_product {
                max_product = product;
            }
        }

        Ok(max_product)
    }
}
