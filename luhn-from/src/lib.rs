use std::fmt::Display;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.code
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .try_fold((0, 0), |(len, checksum), c| {
                c.to_digit(10)
                    .map(|d| if len % 2 == 0 { d } else { d * 2 })
                    .map(|n| (len + 1, checksum + if n > 9 { n - 9 } else { n }))
            })
            .is_some_and(|(len, checksum)| len > 1 && checksum % 10 == 0)
    }
}

impl<T> From<T> for Luhn
where
    T: Display,
{
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
