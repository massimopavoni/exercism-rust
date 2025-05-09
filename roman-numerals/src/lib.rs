use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    symbol: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.symbol)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        Self {
            symbol: {
                let mut min_numeral = String::new();

                for &(value, symbol) in NUMERALS.iter() {
                    while value <= num {
                        min_numeral += symbol;
                        num -= value;
                    }
                }

                min_numeral
            },
        }
    }
}

const NUMERALS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];
