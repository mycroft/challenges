pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows : Vec<Vec<u32>> = vec![];

        for _i in 0usize..self.row_count as usize {
            let mut row = vec![];

            row.push(1);

            if _i == 0 {
                rows.push(row);
                continue;
            }

            for _j in 1usize.._i {
                row.push(rows[_i-1][_j] + rows[_i-1][_j-1]);
            }

            row.push(1);

            rows.push(row);
        }

        rows
    }
}
