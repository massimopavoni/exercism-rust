pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let w_upper: String = w.chars().filter(|c| c.is_uppercase()).collect();
            match w_upper.len() {
                0 => w.chars().next().unwrap().to_uppercase().to_string(),
                l if l == w.len() => w_upper.chars().next().unwrap().to_string(),
                _ => w_upper,
            }
        })
        .collect::<String>()
        .to_uppercase()
}
