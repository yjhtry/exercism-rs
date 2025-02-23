pub struct Matrix {
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self {
            rows: input
                .lines()
                .map(|row| {
                    row.split_whitespace()
                        .map(|cell| cell.parse::<u32>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.rows
            .iter()
            .map(|row| row.get(col_no - 1).cloned())
            .collect()
    }
}
fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_row_from_one_number_matrix() {
        let matrix = Matrix::new("1");
        assert_eq!(matrix.row(1), Some(vec![1]));
    }
    #[test]
    fn can_extract_row() {
        let matrix = Matrix::new("1 2\n3 4");
        assert_eq!(matrix.row(2), Some(vec![3, 4]));
    }
    #[test]
    fn extract_row_where_numbers_have_different_widths() {
        let matrix = Matrix::new("1 2\n10 20");
        assert_eq!(matrix.row(2), Some(vec![10, 20]));
    }
    #[test]
    fn can_extract_row_from_non_square_matrix_with_no_corresponding_column() {
        let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9\n8 7 6");
        assert_eq!(matrix.row(4), Some(vec![8, 7, 6]));
    }
    #[test]
    fn extract_column_from_one_number_matrix() {
        let matrix = Matrix::new("1");
        assert_eq!(matrix.column(1), Some(vec![1]));
    }
    #[test]
    fn can_extract_column() {
        let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
        assert_eq!(matrix.column(3), Some(vec![3, 6, 9]));
    }
    #[test]
    fn can_extract_column_from_non_square_matrix_with_no_corresponding_row() {
        let matrix = Matrix::new("1 2 3 4\n5 6 7 8\n9 8 7 6");
        assert_eq!(matrix.column(4), Some(vec![4, 8, 6]));
    }
    #[test]
    fn extract_column_where_numbers_have_different_widths() {
        let matrix = Matrix::new("89 1903 3\n18 3 1\n9 4 800");
        assert_eq!(matrix.column(2), Some(vec![1903, 3, 4]));
    }
    #[test]
    fn cannot_extract_row_with_no_corresponding_row_in_matrix() {
        let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
        assert_eq!(matrix.row(4), None);
    }
    #[test]
    fn cannot_extract_column_with_no_corresponding_column_in_matrix() {
        let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
        assert_eq!(matrix.column(4), None);
    }
}
