use std::iter::once;

use transpose::transpose_inplace;

pub fn encrypt(input: &str) -> String {
    let mut input: Vec<u8> = input
        .to_ascii_lowercase()
        .bytes()
        .filter(u8::is_ascii_alphanumeric)
        .collect();

    if input.is_empty() {
        return String::new();
    }

    let (cols, rows) = {
        let sqrt = (input.len() as f64).sqrt();

        (sqrt.ceil() as usize, sqrt.round() as usize)
    };

    input.resize_with(rows * cols, || b' ');
    transpose_inplace(&mut input, &mut vec![0; cols], cols, rows);

    input
        .into_iter()
        .enumerate()
        .flat_map(|(i, b)| {
            (if i != 0 && i % rows == 0 {
                Some(' ')
            } else {
                None
            })
            .into_iter()
            .chain(once(b as char))
        })
        .collect()
}
