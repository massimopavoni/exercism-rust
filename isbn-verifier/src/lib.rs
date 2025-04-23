pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");

    if isbn.len() != 10 {
        return false;
    }

    let mut sum = 0;

    for (i, c) in isbn.char_indices() {
        sum += if c.is_numeric() {
            c.to_digit(10).unwrap() * (10 - i as u32)
        } else if i == 9 && c == 'X' {
            10
        } else {
            return false;
        };
    }

    sum % 11 == 0
}
