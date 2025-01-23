use std::ops::ControlFlow;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(sum, count), curr| {
            curr.to_digit(10)
                .map(|v| if count % 2 == 1 { 2 * v } else { v })
                .map(|v| if v > 9 { v - 9 } else { v })
                .map(|v| (sum + v, count + 1))
        })
        .is_some_and(|(sum, count)| sum % 10 == 0 && count > 1)
    // let code = code.replace(" ", "");
    //
    // if code.len() < 2 {
    //     return false;
    // }
    //
    // if code.chars().any(|c| !c.is_ascii_digit()) {
    //     return false;
    // }
    //
    // let sum: u32 = code
    //     .chars()
    //     .rev()
    //     .enumerate()
    //     .map(|(i, c)| match (i + 1) % 2 {
    //         0 => match c.to_digit(10).unwrap() {
    //             x if x < 5 => x * 2,
    //             x => x * 2 - 9,
    //         },
    //         _ => c.to_digit(10).unwrap(),
    //     })
    //     .sum();
    //
    // sum % 10 == 0

    // let mut sum = 0;
    // for (i, c) in chars.chars().rev().enumerate() {
    //     if !c.is_ascii_digit() {
    //         return false;
    //     }
    //
    //     let dc = c.to_digit(10).unwrap();
    //     if (i + 1) % 2 == 0 {
    //         let double = dc * 2;
    //
    //         if double > 9 {
    //             sum += double - 9;
    //         } else {
    //             sum += double;
    //         }
    //     } else {
    //         sum += dc;
    //     }
    // }
    //
    // sum % 10 == 0
}

fn main() {
    let triangular = (1..30).try_fold(0_i8, |prev, x| {
        if let Some(next) = prev.checked_add(x) {
            ControlFlow::Continue(next)
        } else {
            ControlFlow::Break(prev)
        }
    });
    assert_eq!(triangular, ControlFlow::Break(120));

    let triangular = (1..30).try_fold(0_u64, |prev, x| {
        if let Some(next) = prev.checked_add(x) {
            ControlFlow::Continue(next)
        } else {
            ControlFlow::Break(prev)
        }
    });
    assert_eq!(triangular, ControlFlow::Continue(435));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn single_digit_strings_can_not_be_valid() {
        assert!(!is_valid("1"));
    }
    #[test]
    fn a_single_zero_is_invalid() {
        assert!(!is_valid("0"));
    }
    #[test]
    fn a_simple_valid_sin_that_remains_valid_if_reversed() {
        assert!(is_valid("059"));
    }
    #[test]
    fn a_simple_valid_sin_that_becomes_invalid_if_reversed() {
        assert!(is_valid("59"));
    }
    #[test]
    fn a_valid_canadian_sin() {
        assert!(is_valid("055 444 285"));
    }
    #[test]
    fn invalid_canadian_sin() {
        assert!(!is_valid("055 444 286"));
    }
    #[test]
    fn invalid_credit_card() {
        assert!(!is_valid("8273 1232 7352 0569"));
    }
    #[test]
    fn invalid_long_number_with_an_even_remainder() {
        assert!(!is_valid("1 2345 6789 1234 5678 9012"));
    }
    #[test]
    fn invalid_long_number_with_a_remainder_divisible_by_5() {
        assert!(!is_valid("1 2345 6789 1234 5678 9013"));
    }
    #[test]
    fn valid_number_with_an_even_number_of_digits() {
        assert!(is_valid("095 245 88"));
    }
    #[test]
    fn valid_number_with_an_odd_number_of_spaces() {
        assert!(is_valid("234 567 891 234"));
    }
    #[test]
    fn valid_strings_with_a_non_digit_added_at_the_end_become_invalid() {
        assert!(!is_valid("059a"));
    }
    #[test]
    fn valid_strings_with_punctuation_included_become_invalid() {
        assert!(!is_valid("055-444-285"));
    }
    #[test]
    fn valid_strings_with_symbols_included_become_invalid() {
        assert!(!is_valid("055# 444$ 285"));
    }
    #[test]
    fn single_zero_with_space_is_invalid() {
        assert!(!is_valid(" 0"));
    }
    #[test]
    fn more_than_a_single_zero_is_valid() {
        assert!(is_valid("0000 0"));
    }
    #[test]
    fn input_digit_9_is_correctly_converted_to_output_digit_9() {
        assert!(is_valid("091"));
    }
    #[test]
    fn very_long_input_is_valid() {
        assert!(is_valid("9999999999 9999999999 9999999999 9999999999"));
    }
    #[test]
    fn valid_luhn_with_an_odd_number_of_digits_and_non_zero_first_digit() {
        assert!(is_valid("109"));
    }
    #[test]
    fn using_ascii_value_for_non_doubled_non_digit_isn_t_allowed() {
        assert!(!is_valid("055b 444 285"));
    }
    #[test]
    fn using_ascii_value_for_doubled_non_digit_isn_t_allowed() {
        assert!(!is_valid(":9"));
    }
    #[test]
    fn non_numeric_non_space_char_in_the_middle_with_a_sum_that_s_divisible_by_10_isn_t_allowed() {
        assert!(!is_valid("59%59"));
    }
}
