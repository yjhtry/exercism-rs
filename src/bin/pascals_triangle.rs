pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..=self.0).fold(vec![], |mut res, curr| {
            match res.last() {
                None => vec![vec![1]],
                Some(last_row) => {
                    let mut curr_row = vec![];
                    for idx in 0..curr {
                        // get value from last_level
                        let mut value = 0;
                        if idx > 0 {
                            if let Some(v) = last_row.get((idx - 1) as usize) {
                                value += *v;
                            }
                        }
                        if idx < curr {
                            if let Some(v) = last_row.get(idx as usize) {
                                value += *v;
                            }
                        }

                        curr_row.push(value)
                    }
                    res.push(curr_row);
                    res
                }
            }
        })
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn zero_rows() {
        let pt = PascalsTriangle::new(0);
        let expected: Vec<Vec<u32>> = vec![];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    fn single_row() {
        let pt = PascalsTriangle::new(1);
        let expected: Vec<Vec<u32>> = vec![vec![1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    fn two_rows() {
        let pt = PascalsTriangle::new(2);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    fn three_rows() {
        let pt = PascalsTriangle::new(3);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    fn four_rows() {
        let pt = PascalsTriangle::new(4);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    fn five_rows() {
        let pt = PascalsTriangle::new(5);
        let expected: Vec<Vec<u32>> =
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![
                1, 4, 6, 4, 1,
            ]];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    fn six_rows() {
        let pt = PascalsTriangle::new(6);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }
    #[test]
    fn ten_rows() {
        let pt = PascalsTriangle::new(10);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }
}
