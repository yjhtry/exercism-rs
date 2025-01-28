pub fn is_armstrong_number(num: u32) -> bool {
    // 获取num的位数
    let digits = num_digits(num);

    let mut sum = 0;
    let mut copy = num;
    while copy > 0 {
        sum += (copy % 10).pow(digits);

        copy /= 10;
    }

    num == sum
}

fn num_digits(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }

    num.ilog10() + 1
}

fn main() {
    is_armstrong_number(9475);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(1), 1);
        assert_eq!(num_digits(9), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(19), 2);
        assert_eq!(num_digits(100), 3);
        assert_eq!(num_digits(9475), 4);
    }

    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }
    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }
    #[test]
    fn there_are_no_two_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }
    #[test]
    fn three_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    fn three_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }
    #[test]
    fn four_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_474))
    }
    #[test]
    fn four_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_475))
    }
    #[test]
    fn seven_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }
    #[test]
    fn seven_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_926_314))
    }
}
