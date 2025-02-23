use std::ops::Rem;

pub struct Matcher<T> {
    matcher: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(matcher: fn(T) -> bool, subs: S) -> Matcher<T> {
        Self {
            matcher,
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Clone + ToString,
{
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);

        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |v| {
            let output: String = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(v.clone()))
                .map(|m| m.subs.clone())
                .collect();

            if output.is_empty() {
                v.to_string()
            } else {
                output
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<Output = T> + PartialEq + ToString + Copy + From<u8> + 'static,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple() {
        let actual = fizz_buzz::<i32>().apply(1..=16).collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn u8() {
        let actual = fizz_buzz::<u8>().apply(1_u8..=16).collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn u64() {
        let actual = fizz_buzz::<u64>().apply(1_u64..=16).collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];
        let actual = fizz_buzz::<i32>()
            .apply(collatz_12.iter().cloned())
            .collect::<Vec<_>>();
        let expected = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn custom() {
        let expected = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];
        let fizzer: Fizzy<i32> = Fizzy::new()
            .add_matcher(Matcher::new(|n: i32| n % 5 == 0, "Buzz"))
            .add_matcher(Matcher::new(|n: i32| n % 3 == 0, "Fizz"))
            .add_matcher(Matcher::new(|n: i32| n % 7 == 0, "Bam"));
        let actual = fizzer.apply(1..=16).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
    #[test]
    fn f64() {
        // a tiny bit more complicated becuase range isn't natively implemented on floats
        let actual = fizz_buzz::<f64>()
            .apply(std::iter::successors(Some(1.0), |prev| Some(prev + 1.0)))
            .take(16)
            .collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn minimal_generic_bounds() {
        use std::fmt;
        use std::ops::{Add, Rem};
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        struct Fizzable(u8);
        impl From<u8> for Fizzable {
            fn from(i: u8) -> Fizzable {
                Fizzable(i)
            }
        }
        impl fmt::Display for Fizzable {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let Fizzable(n) = self;
                write!(f, "{n}")
            }
        }
        impl Add for Fizzable {
            type Output = Fizzable;
            fn add(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 + n2)
            }
        }
        impl Rem for Fizzable {
            type Output = Fizzable;
            fn rem(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 % n2)
            }
        }
        let actual = fizz_buzz::<Fizzable>()
            .apply(std::iter::successors(Some(Fizzable(1)), |prev| {
                Some(*prev + 1.into())
            }))
            .take(16)
            .collect::<Vec<_>>();
        let expected = [
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        assert_eq!(actual, expected);
    }
}
