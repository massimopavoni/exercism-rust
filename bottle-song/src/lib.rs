const LOWERCASE_NUMBERS: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

const UPPERCASE_NUMBERS: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut text = String::new();

    for bottles in (start_bottles - take_down + 1..=start_bottles).rev() {
        text += &format!(
            "\n{0} green bottle{1} hanging on the wall,\n\
            {0} green bottle{1} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {2} green bottle{3} hanging on the wall.\n",
            UPPERCASE_NUMBERS[bottles as usize],
            if bottles == 1 { "" } else { "s" },
            LOWERCASE_NUMBERS[(bottles - 1) as usize],
            if bottles == 2 { "" } else { "s" }
        );
    }

    text
}
