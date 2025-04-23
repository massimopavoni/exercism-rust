pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<_>> = Vec::with_capacity(row_count as usize);

        for i in 0..row_count {
            let mut row = vec![1; i as usize + 1];

            for j in 1..i {
                row[j as usize] =
                    rows[i as usize - 1][j as usize - 1] + rows[i as usize - 1][j as usize];
            }

            rows.push(row);
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
