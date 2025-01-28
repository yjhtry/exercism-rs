fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    (2..=(n / 2)).all(|i| n % i != 0)
}
pub fn nth(n: u32) -> u32 {
    (2_u32..).filter(|&s| is_prime(s)).nth(n as usize).unwrap()
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn first_prime() {
        let output = nth(0);
        let expected = 2;
        assert_eq!(output, expected);
    }
    #[test]
    fn second_prime() {
        let output = nth(1);
        let expected = 3;
        assert_eq!(output, expected);
    }
    #[test]
    fn sixth_prime() {
        let output = nth(5);
        let expected = 13;
        assert_eq!(output, expected);
    }
    #[test]
    fn big_prime() {
        let output = nth(10_000);
        let expected = 104_743;
        assert_eq!(output, expected);
    }
}
