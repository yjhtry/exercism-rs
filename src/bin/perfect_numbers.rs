use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }
    match num.cmp(&((1..=(num / 2)).filter(|n| num % n == 0).sum())) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Deficient),
        Ordering::Less => Some(Classification::Abundant),
    }
}

fn main() {
    let list: Vec<i32> = (1..=(24 / 2)).filter(|n| 24 % n == 0).collect();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn smallest_perfect_number_is_classified_correctly() {
        let input = 6;
        let output = classify(input);
        let expected = Some(Classification::Perfect);
        assert_eq!(output, expected);
    }
    #[test]
    fn medium_perfect_number_is_classified_correctly() {
        let input = 28;
        let output = classify(input);
        let expected = Some(Classification::Perfect);
        assert_eq!(output, expected);
    }
    #[test]
    fn large_perfect_number_is_classified_correctly() {
        let input = 33_550_336;
        let output = classify(input);
        let expected = Some(Classification::Perfect);
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_abundant_number_is_classified_correctly() {
        let input = 12;
        let output = classify(input);
        let expected = Some(Classification::Abundant);
        assert_eq!(output, expected);
    }
    #[test]
    fn medium_abundant_number_is_classified_correctly() {
        let input = 30;
        let output = classify(input);
        let expected = Some(Classification::Abundant);
        assert_eq!(output, expected);
    }
    #[test]
    fn large_abundant_number_is_classified_correctly() {
        let input = 33_550_335;
        let output = classify(input);
        let expected = Some(Classification::Abundant);
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_prime_deficient_number_is_classified_correctly() {
        let input = 2;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_non_prime_deficient_number_is_classified_correctly() {
        let input = 4;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }
    #[test]
    fn medium_deficient_number_is_classified_correctly() {
        let input = 32;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }
    #[test]
    fn large_deficient_number_is_classified_correctly() {
        let input = 33_550_337;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }
    #[test]
    fn edge_case_no_factors_other_than_itself_is_classified_correctly() {
        let input = 1;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }
    #[test]
    fn zero_is_rejected_as_it_is_not_a_positive_integer() {
        let input = 0;
        let output = classify(input);
        assert!(output.is_none());
    }
}
