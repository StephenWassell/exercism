pub struct PascalsTriangle {
    count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::<Vec<u32>>::new();
        for i in 0..self.count {
            rows.push(Vec::<u32>::new());
        }
        rows
    }
}
