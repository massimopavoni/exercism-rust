trait VowelCheck {
    fn is_vowel(&self) -> bool;
}

impl VowelCheck for char {
    fn is_vowel(&self) -> bool {
        matches!(self, 'a' | 'e' | 'i' | 'o' | 'u')
    }
}

fn translate_word(input: &str) -> String {
    if input.starts_with("xr") || input.starts_with("yt") {
        return format!("{}ay", input);
    }

    let vowel_index = input.chars().position(|c| c.is_vowel()).or_else(|| {
        input
            .chars()
            .enumerate()
            .position(|(i, c)| c == 'y' && i > 0)
    });

    if let Some(vowel_index) = vowel_index {
        if vowel_index == 0 {
            return format!("{}ay", input);
        }

        if let Some(qu_index) = input.find("qu") {
            return format!("{}{}ay", &input[qu_index + 2..], &input[..qu_index + 2]);
        }

        return format!("{}{}ay", &input[vowel_index..], &input[..vowel_index]);
    }

    "".to_owned()
}

pub fn translate(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}
