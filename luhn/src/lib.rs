/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(len, checksum), c| {
            c.to_digit(10)
                .map(|d| if len % 2 == 0 { d } else { d * 2 })
                .map(|n| (len + 1, checksum + if n > 9 { n - 9 } else { n }))
        })
        .map_or(false, |(len, checksum)| len > 1 && checksum % 10 == 0)
}
