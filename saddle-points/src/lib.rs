pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();
    let row_count = input.len();

    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if row.iter().all(|x| x <= cell) && (0..row_count).all(|x| input[x][j] >= *cell) {
                saddle_points.push((i, j));
            }
        }
    }

    saddle_points
}
