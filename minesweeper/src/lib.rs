const MINE_BYTE: u8 = b'*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res = Vec::new();

    for r in 0..minefield.len() {
        let mut row = String::new();

        for c in 0..minefield[0].len() {
            if minefield[r].as_bytes()[c] == MINE_BYTE {
                row.push('*');
            } else {
                let count = neighbors_count(minefield, r, c);

                row.push(if count == 0 {
                    ' '
                } else {
                    char::from_digit(count as u32, 10).expect("Invalid digit")
                });
            }
        }

        res.push(row);
    }

    res
}

fn neighbors_count(minefield: &[&str], x: usize, y: usize) -> usize {
    let y_range = y.saturating_sub(1)..y + 2;
    (x.saturating_sub(1)..x + 2)
        .filter_map(move |x| minefield.get(x))
        .flat_map(|row| y_range.clone().filter_map(move |y| row.as_bytes().get(y)))
        .filter(|&&c| c == MINE_BYTE)
        .count()
}
