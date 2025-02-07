use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64, factors: HashSet<(u64, u64)>) -> Self {
        Self { value, factors }
    }
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let map = (min..=max)
        .combinations_with_replacement(2)
        .map(|pairs| (pairs[0], pairs[1]))
        .filter(|(x, y)| is_palindrome(x * y))
        .fold(HashMap::new(), |mut h, (x, y)| {
            h.entry(x * y)
                .and_modify(|v: &mut HashSet<(u64, u64)>| {
                    v.insert((x, y));
                })
                .or_insert_with(|| HashSet::from([(x, y)]));
            h
        });
    if map.is_empty() {
        return None;
    }
    let mut keys = map.keys().cloned().collect::<Vec<u64>>();
    keys.sort();

    let (min_value, max_value) = (*keys.first().unwrap(), *keys.last().unwrap());

    let (min_set, max_set) = (map.get(&min_value), map.get(&max_value));

    Some((
        Palindrome::new(min_value, min_set.unwrap().clone()),
        Palindrome::new(max_value, max_set.unwrap().clone()),
    ))
}

fn is_palindrome(n: u64) -> bool {
    if n % 10 == 0 && n != 0 {
        return false; // 负数和以 0 结尾的数（非 0）一定不是回文数
    }

    let mut num = n;
    let mut reversed = 0;

    while num > reversed {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    // 当数字长度为奇数时，例如 12321，reversed 最后会变成 123，num 变成 12，直接忽略中间位
    num == reversed || num == reversed / 10
}

fn main() {
    let iter = (1..=9)
        .combinations_with_replacement(2)
        .map(|pairs| pairs[0] * pairs[1]);

    for value in iter {
        print!("{} ", value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_the_smallest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 1);
        assert_eq!(pal.into_factors(), HashSet::from([(1, 1)]));
    }
    #[test]
    fn find_the_largest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 9);
        assert_eq!(pal.into_factors(), HashSet::from([(1, 9), (3, 3)]));
    }
    #[test]
    fn find_the_smallest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 121);
        assert_eq!(pal.into_factors(), HashSet::from([(11, 11)]));
    }
    #[test]
    fn find_the_largest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 9009);
        assert_eq!(pal.into_factors(), HashSet::from([(91, 99)]));
    }
    #[test]
    fn find_the_smallest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 10201);
        assert_eq!(pal.into_factors(), HashSet::from([(101, 101)]));
    }
    #[test]
    fn find_the_largest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 906609);
        assert_eq!(pal.into_factors(), HashSet::from([(913, 993)]));
    }
    #[test]
    fn find_the_smallest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 1002001);
        assert_eq!(pal.into_factors(), HashSet::from([(1001, 1001)]));
    }
    #[test]
    fn find_the_largest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 99000099);
        assert_eq!(pal.into_factors(), HashSet::from([(9901, 9999)]));
    }
    #[test]
    fn empty_result_for_smallest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(1002, 1003);
        assert!(output.is_none());
    }
    #[test]
    fn empty_result_for_largest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(15, 15);
        assert!(output.is_none());
    }
    #[test]
    fn error_result_for_smallest_if_min_is_more_than_max() {
        let output = palindrome_products(10000, 1);
        assert!(output.is_none());
    }
    #[test]
    fn error_result_for_largest_if_min_is_more_than_max() {
        let output = palindrome_products(2, 1);
        assert!(output.is_none());
    }
    #[test]
    fn smallest_product_does_not_use_the_smallest_factor() {
        let output = palindrome_products(3215, 4000);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 10988901);
        assert_eq!(pal.into_factors(), HashSet::from([(3297, 3333)]));
    }
}
