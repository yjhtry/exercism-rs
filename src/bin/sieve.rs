// pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
//     (0..=upper_bound).filter(|n| is_prime(*n)).collect()
// }
// fn is_prime(n: u64) -> bool {
//     if n <= 1 {
//         return false;
//     }
//
//     if n == 2 || n == 3 {
//         return true;
//     }
//
//     if n % 2 == 0 {
//         return false;
//     }
//
//     let sqrt = (n as f64).sqrt() as u64;
//
//     for i in (3..=sqrt).step_by(2) {
//         if n % i == 0 {
//             return false;
//         }
//     }
//
//     true
// }

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (0..=upper_bound).map(Option::from).collect();

    let upper_bound = upper_bound as usize;
    (2..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()? as usize;
            ((prime * prime)..=upper_bound)
                .step_by(prime)
                .for_each(|j| numbers[j] = None);
            Some(prime as u64)
        })
        .collect()
}
fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn no_primes_under_two() {
        let input = 1;
        let output = primes_up_to(input);
        let expected = [];
        assert_eq!(output, expected);
    }
    #[test]
    fn find_first_prime() {
        let input = 2;
        let output = primes_up_to(input);
        let expected = [2];
        assert_eq!(output, expected);
    }
    #[test]
    fn find_primes_up_to_10() {
        let input = 10;
        let output = primes_up_to(input);
        let expected = [2, 3, 5, 7];
        assert_eq!(output, expected);
    }
    #[test]
    fn limit_is_prime() {
        let input = 13;
        let output = primes_up_to(input);
        let expected = [2, 3, 5, 7, 11, 13];
        assert_eq!(output, expected);
    }
    #[test]
    fn find_primes_up_to_1000() {
        let input = 1000;
        let output = primes_up_to(input);
        let expected = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ];
        assert_eq!(output, expected);
    }
}
