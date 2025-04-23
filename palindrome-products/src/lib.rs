#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

fn is_palindrome(n: u64) -> bool {
    if n % 10 == 0 {
        return false;
    };

    let mut reverse = 0;
    let mut r = n;

    while r > 0 {
        reverse = reverse * 10 + r % 10;
        r /= 10;
    }

    n == reverse
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome = None;
    let mut max_palindrome = None;

    for i in min..=max {
        if i % 10 == 0 {
            continue;
        }

        for j in min..=max {
            if j % 10 == 0 {
                continue;
            }

            let p = i * j;

            if is_palindrome(p) {
                if p < min_palindrome.unwrap_or(u64::MAX) {
                    min_palindrome = Some(p);
                }

                if p > max_palindrome.unwrap_or(u64::MIN) {
                    max_palindrome = Some(p);
                }
            }
        }
    }

    Some((
        Palindrome::new(min_palindrome?).unwrap(),
        Palindrome::new(max_palindrome?).unwrap(),
    ))
}
