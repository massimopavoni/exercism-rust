pub fn get_diamond(c: char) -> Vec<String> {
    let half_width = ((c as u8 - b'A') * 2 + 1) as usize / 2;

    (b'A'..=c as u8)
        .chain((b'A'..c as u8).rev())
        .map(|l| {
            let l_count = (l - b'A') as usize;
            format!(
                "{0}{1}{2}{3}{0}",
                " ".repeat(half_width - l_count),
                l as char,
                " ".repeat((l_count * 2).saturating_sub(1)),
                if l_count == 0 {
                    String::new()
                } else {
                    (l as char).to_string()
                }
            )
        })
        .collect()
}
