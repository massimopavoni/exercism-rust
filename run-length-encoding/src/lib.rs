use itertools::Itertools;

pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    for (c, group) in source.chars().chunk_by(|&c| c).into_iter() {
        let count = group.count();

        if count > 1 {
            result.push_str(&count.to_string());
        }

        result.push(c);
    }

    result
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut count = 0;

    for c in source.chars() {
        if let Some(d) = c.to_digit(10) {
            count = count * 10 + d;
        } else if count == 0 {
            result.push(c);
        } else {
            result.push_str(&c.to_string().repeat(count as usize));
            count = 0;
        }
    }

    result
}
