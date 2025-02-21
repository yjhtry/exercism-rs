#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    Ok(string_digits
        .chars()
        .collect::<Vec<char>>()
        .windows(span)
        .map(|v| {
            Iterator::fold(v.iter(), Ok(1), |res, c| {
                c.to_digit(10)
                    .ok_or(Error::InvalidDigit(*c))
                    .map(|v| v as u64 * res.unwrap())
            })
        })
        .collect::<Result<Vec<u64>, Error>>()?
        .into_iter()
        .max()
        .unwrap())
}

fn main() {
    // Iterator<Item = Option<any>>
    // Iterator<Item = Result<any>>
    // 迭代器正常迭代
    // 但是消费可能会因为消费的类型提前终止并返回None活Err

    let nums = vec![Some(1), Some(2), None, Some(4)];
    let iter = nums.into_iter();

    for val in iter {
        println!("{:?}", val);
    }

    let nums = vec![Some(1), Some(2), None, Some(4)];
    let nss = nums.into_iter().collect::<Option<Vec<i32>>>();

    println!("collect option {:?}", nss);

    let nums = vec![Ok(1), Ok(2), Err(""), Ok(4)];
    let iter = nums.into_iter();

    for val in iter {
        println!("{:?}", val);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn return_is_a_result() {
        assert!(lsp("29", 2).is_ok());
    }
    #[test]
    fn find_the_largest_product_when_span_equals_length() {
        assert_eq!(Ok(18), lsp("29", 2));
    }
    #[test]
    fn find_the_largest_product_of_two_with_numbers_in_order() {
        assert_eq!(Ok(72), lsp("0123456789", 2));
    }
    #[test]
    fn find_the_largest_product_of_two_with_numbers_not_in_order() {
        assert_eq!(Ok(48), lsp("576802143", 2));
    }
    #[test]
    fn find_the_largest_product_of_three_with_numbers_in_order() {
        assert_eq!(Ok(504), lsp("0123456789", 3));
    }
    #[test]
    fn find_the_largest_product_of_three_with_numbers_not_in_order() {
        assert_eq!(Ok(270), lsp("1027839564", 3));
    }
    #[test]
    fn find_the_largest_product_of_five_with_numbers_in_order() {
        assert_eq!(Ok(15_120), lsp("0123456789", 5));
    }
    #[test]
    fn span_of_six_in_a_large_number() {
        assert_eq!(
            Ok(23_520),
            lsp("73167176531330624919225119674426574742355349194934", 6)
        );
    }
    #[test]
    fn returns_zero_if_number_is_zeros() {
        assert_eq!(Ok(0), lsp("0000", 2));
    }
    #[test]
    fn returns_zero_if_all_products_are_zero() {
        assert_eq!(Ok(0), lsp("99099", 3));
    }
    #[test]
    fn a_span_is_longer_than_number_is_an_error() {
        assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
    }
    #[test]
    fn empty_string_and_non_zero_span_is_an_error() {
        assert_eq!(Err(Error::SpanTooLong), lsp("", 1));
    }
    #[test]
    fn a_string_with_non_digits_is_an_error() {
        assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
    }
}
