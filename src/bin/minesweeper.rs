#[rustfmt::skip]
static ROUND_COORDS: &[(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1), 
    (-1,  0),          (1, 0), 
    (-1,  1), (0,  1), (1, 1),
];

pub fn annotate(input: &[&str]) -> Vec<String> {
    let height = input.len();
    if height == 0 {
        return Vec::new();
    };

    let width = input[0].len();

    (0..height)
        .map(|h| {
            let row = input[h].as_bytes();
            (0..width)
                .map(|w| {
                    if row[w] == b'*' {
                        '*'
                    } else {
                        match ROUND_COORDS
                            .iter()
                            .map(|&(ox, oy)| (w as i32 + ox, h as i32 + oy))
                            .filter(|&(x, y)| {
                                (0 <= x && x < (width as i32)) && (0 <= y && y < (height as i32))
                            })
                            .filter(|&(x, y)| input[y as usize].as_bytes()[x as usize] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => ((n as u8) + b'0').into(),
                        }
                    }
                })
                .collect()
        })
        .collect()

    // if height == 0 {
    //     return Vec::new();
    // };
    //
    // let width = input[0].len();
    // let mut result = Vec::with_capacity(height);
    //
    // for (idx1, row) in input.iter().enumerate() {
    //     let bytes = row.as_bytes();
    //     let mut new_row = String::with_capacity(width);
    //     for (idx2, b) in bytes.iter().enumerate() {
    //         if *b == b'*' {
    //             new_row.push('*');
    //             continue;
    //         }
    //
    //         let mut count = 0_u8;
    //         // check cells
    //         for i in -1_i32..=1 {
    //             for j in -1_i32..=1 {
    //                 let r_idx = (idx1 as i32) + i;
    //                 let c_idx = (idx2 as i32) + j;
    //
    //                 // check cell
    //                 if r_idx >= 0 && r_idx < (height as i32) && c_idx >= 0 && c_idx < (width as i32)
    //                 {
    //                     let sb = input[r_idx as usize]
    //                         .as_bytes()
    //                         .get(c_idx as usize)
    //                         .unwrap();
    //
    //                     if *sb == b'*' {
    //                         count += 1;
    //                     }
    //                 }
    //             }
    //         }
    //
    //         if count == 0 {
    //             new_row.push(' ');
    //         } else {
    //             new_row.push((count + b'0').into());
    //         }
    //     }
    //
    //     result.push(new_row)
    // }
    //
    // result
}
fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn no_rows() {
        let input = &[];
        let expected: &[&str] = &[];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn no_columns() {
        let input = &[""];
        let expected = &[""];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn no_mines() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        "   ",
        "   ",
    ], &[
        "   ",
        "   ",
        "   ",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn minefield_with_only_mines() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "***",
        "***",
    ], &[
        "***",
        "***",
        "***",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "   ",
        " * ",
        "   ",
    ], &[
        "111",
        "1*1",
        "111",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "***",
        "* *",
        "***",
    ], &[
        "***",
        "*8*",
        "***",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn horizontal_line() {
        let input = &[" * * "];
        let expected = &["1*2*1"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn horizontal_line_mines_at_edges() {
        let input = &["*   *"];
        let expected = &["*1 1*"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        " ",
        "*",
        " ",
        "*",
        " ",
    ], &[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "*",
        " ",
        " ",
        " ",
        "*",
    ], &[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn cross() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        "  *  ",
        "  *  ",
        "*****",
        "  *  ",
        "  *  ",
    ], &[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
    #[test]
    fn large_minefield() {
        #[rustfmt::skip]
    let (input, expected) = (&[
        " *  * ",
        "  *   ",
        "    * ",
        "   * *",
        " *  * ",
        "      ",
    ], &[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
}
