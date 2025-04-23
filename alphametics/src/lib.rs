use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = input.replace(" ", "");

    let (letters, factors): (Vec<_>, Vec<_>) = {
        let mut factors = HashMap::new();
        let mut sign = -1;
        let mut pos = 0;

        for c in input.chars().rev() {
            match c {
                '=' => {
                    sign = 1;
                    pos = 0
                }
                '+' => pos = 0,
                _ => {
                    *factors.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                    pos += 1;
                }
            }
        }

        factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip()
    };

    let non_zero_letters: Vec<_> = input
        .split(&['+', '='])
        .filter_map(|w| w.chars().next())
        .collect();

    (0..10).permutations(letters.len()).find_map(|permutation| {
        if permutation
            .iter()
            .enumerate()
            .any(|(i, &d)| d == 0 && non_zero_letters.contains(&letters[i]))
        {
            return None;
        }

        if permutation
            .iter()
            .enumerate()
            .map(|(i, d)| d * factors[i])
            .sum::<i64>()
            == 0
        {
            return Some(HashMap::from_iter(
                permutation
                    .iter()
                    .enumerate()
                    .map(|(i, &d)| (letters[i], d as u8)),
            ));
        }

        None
    })
}
