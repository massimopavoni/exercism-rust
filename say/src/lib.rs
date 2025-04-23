const SMALL: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: &[&str] = &[
    "PANIC", "PANIC", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const MAGNITUDE: &[&str] = &[
    "PANIC",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_owned();
    }

    let mut n = n;
    let mut result = String::new();

    while n != 0 {
        match n {
            0..20 => {
                result += SMALL[n as usize];
                break;
            }
            20..100 => {
                result += TENS[n as usize / 10];
                n %= 10;

                if n != 0 {
                    result += "-";
                }
            }
            100..1_000 => {
                result += &format!("{} hundred", SMALL[n as usize / 100]);
                n %= 100;

                if n != 0 {
                    result += " ";
                }
            }
            _ => {
                let mut top = n;
                let mut magnitude = 0;
                let mut magnitude_pow = 1;

                while top >= 1_000 {
                    top /= 1_000;
                    magnitude += 1;
                    magnitude_pow *= 1_000;
                }

                result += &encode(top);
                n %= magnitude_pow;
                result += &format!(" {}", MAGNITUDE[magnitude as usize]);

                if n != 0 {
                    result += " ";
                }
            }
        }
    }

    result
}
