use std::iter;

//  1  2  3 4     1  2  3  4
// 12 13 14 5     5  6  7  8
// 11 16 15 6     9 10 11 12
// 10  9  8 7    13 14 15 16
// pub fn spiral_matrix(n: usize) -> Vec<Vec<u32>> {
//     if n < 1 {
//         return vec![];
//     }
//     let mut matrix = vec![vec![0; n]; n];
//
//     let (mut left, mut right) = (0, n - 1);
//     let (mut top, mut bottom) = (0, n - 1);
//     let mut num = 1_u32;
//
//     while num <= (n * n) as u32 {
//         // 从左到右
//         for i in left..=right {
//             matrix[top][i] = num;
//             num += 1;
//         }
//         top += 1;
//
//         // 从上到下
//         (top..=bottom).for_each(|i| {
//             matrix[i][right] = num;
//             num += 1;
//         });
//         right = right.saturating_sub(1);
//
//         // 从右到左
//         if top <= bottom {
//             for i in (left..=right).rev() {
//                 matrix[bottom][i] = num;
//                 num += 1;
//             }
//             bottom = bottom.saturating_sub(1);
//         }
//
//         // 从下到上
//         if left <= right {
//             for i in (top..=bottom).rev() {
//                 matrix[i][left] = num;
//                 num += 1;
//             }
//             left += 1;
//         }
//     }
//
//     matrix
// }

pub const VECTORS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// Other
// for i in (0..size + size) {
//   for j in (0..(size + size - i) / 2) {
//     // if size is 4, j: 4, 3, 3, 2, 2, 1, 1
//     x += VECTORS[i%4].0
//     y += VECTORS[i%4].1
//   }
// }
//

// turn size * 2 - 1 times
// Example size is 4, move 4, 3, 3, 2, 2, 1, 1
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let (mut x, mut y, mut n) = (-1, 0, 1..);
    let mut moves = VECTORS.iter().cycle();

    for (move_x, move_y) in iter::once(size)
        .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
        .flat_map(|steps| iter::repeat(moves.next().unwrap()).take(steps as usize))
    {
        x += move_x;
        y += move_y;
        matrix[y as usize][x as usize] = n.next().unwrap()
    }

    matrix
}

fn main() {
    spiral_matrix(4);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_spiral() {
        let input = 0;
        let output = spiral_matrix(input);
        let expected: [[u32; 0]; 0] = [];
        assert_eq!(output, expected);
    }
    #[test]
    fn trivial_spiral() {
        let input = 1;
        let output = spiral_matrix(input);
        let expected: [[u32; 1]; 1] = [[1]];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_2() {
        let input = 2;
        let output = spiral_matrix(input);
        let expected: [[u32; 2]; 2] = [[1, 2], [4, 3]];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_3() {
        let input = 3;
        let output = spiral_matrix(input);
        let expected: [[u32; 3]; 3] = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_4() {
        let input = 4;
        let output = spiral_matrix(input);
        let expected: [[u32; 4]; 4] = [[1, 2, 3, 4], [12, 13, 14, 5], [11, 16, 15, 6], [
            10, 9, 8, 7,
        ]];
        assert_eq!(output, expected);
    }
    #[test]
    fn spiral_of_size_5() {
        let input = 5;
        let output = spiral_matrix(input);
        let expected: [[u32; 5]; 5] = [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ];
        assert_eq!(output, expected);
    }
}
