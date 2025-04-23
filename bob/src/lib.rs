pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    match (
        message.chars().any(char::is_alphabetic),
        message.ends_with("?"),
        message.to_uppercase() == message,
    ) {
        (true, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (true, false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
