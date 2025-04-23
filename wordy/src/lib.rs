use regex::{Captures, Regex};

pub fn answer(command: &str) -> Option<i32> {
    let command = Regex::new(
        "(What is|plus|minus|multiplied by|divided by|raised to the (\\d+)(st|nd|rd|th) power|\\?)",
    )
    .expect("Expected valid regex")
    .replace_all(command, |captures: &Captures| match &captures[1] {
        "What is" | "?" => "".to_string(),
        "plus" => "+".to_string(),
        "minus" => "-".to_string(),
        "multiplied by" => "*".to_string(),
        "divided by" => "/".to_string(),
        _ => {
            match (
                captures[2]
                    .as_bytes()
                    .last()
                    .expect("Expected last character"),
                captures[3].as_bytes()[0],
            ) {
                (b'1', b's') | (b'2', b'n') | (b'3', b'r') | (_, b't') => {
                    format!("^ {}", &captures[2])
                }
                _ => "invalid".to_string(),
            }
        }
    });
    let command = command.split_ascii_whitespace().collect::<Vec<_>>();

    if command.len() % 2 == 0 {
        return None;
    }

    let mut result: i32 = command[0].parse().ok()?;

    for op in command[1..].chunks(2) {
        let n: i32 = op[1].parse().ok()?;
        match op[0] {
            "+" => result += n,
            "-" => result -= n,
            "*" => result *= n,
            "/" => result /= n,
            "^" => result = result.pow(n as u32),
            _ => return None,
        }
    }

    Some(result)
}
