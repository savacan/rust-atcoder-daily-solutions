pub struct FenwickTree {
    pub tree: Vec<i64>,
}

impl FenwickTree {
    pub fn new(len: usize) -> Self {
        Self { tree: vec![0; len] }
    }

    pub fn add(&mut self, index: usize, value: i64) {
        let mut n = index + 1;
        while n <= self.tree.len() {
            self.tree[n - 1] += value;
            n += n & n.wrapping_neg()
        }
    }

    pub fn sum(&self, index: usize) -> i64 {
        let mut sum = 0;
        let mut n = index + 1;
        while n > 0 {
            sum += self.tree[n - 1];
            n -= n & n.wrapping_neg()
        }
        sum
    }
}
impl From<Vec<i64>> for FenwickTree {
    fn from(val: Vec<i64>) -> Self {
        let mut tree = Self::new(val.len());
        for (i, v) in val.into_iter().enumerate() {
            tree.add(i, v);
        }
        tree
    }
}
