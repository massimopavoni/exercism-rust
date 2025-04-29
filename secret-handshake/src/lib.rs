pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = vec![];

    if n & 0b1 != 0 {
        actions.push("wink");
    }

    if n & 0b10 != 0 {
        actions.push("double blink");
    }

    if n & 0b100 != 0 {
        actions.push("close your eyes");
    }

    if n & 0b1000 != 0 {
        actions.push("jump");
    }

    if n & 0b10000 != 0 {
        actions.reverse();
    }

    actions
}
