pub fn number(user_number: &str) -> Option<String> {
    let number: Vec<_> = user_number
        .chars()
        .filter(char::is_ascii_digit)
        .skip_while(|&c| c == '1')
        .collect();

    (number.len() == 10 && number[0] > '1' && number[3] > '1')
        .then_some(number.into_iter().collect())
}
