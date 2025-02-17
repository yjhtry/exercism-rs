#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .rev()
        .fold(vec![], |mut res, curr| {
            res.push(*curr as u8 & 0x7f);
            let mut n = *curr >> 7;
            while n != 0 {
                res.push(n as u8 & 0x7f | 1 << 7);
                n >>= 7
            }
            res
        })
        .into_iter()
        .rev()
        .collect()
}
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    bytes
        .chunk_by(|a, _| (*a & 1 << 7) != 0)
        .map(|v| {
            if v.last().unwrap() & 1 << 7 != 0 {
                return Err(Error::IncompleteNumber);
            }
            Ok(v.iter()
                .fold((0, (v.len() - 1) as u32), |(sum, idx), curr| {
                    (
                        sum + (((*curr | 1 << 7) ^ 1 << 7) as u32) * 2_u32.pow(7 * idx),
                        idx - idx.min(1),
                    )
                })
                .0)
        })
        .collect()
}
fn main() {
    println!("{:?}", to_bytes(&[2097152]))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn zero() {
        let input = &[0];
        let output = to_bytes(input);
        let expected = vec![0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn arbitrary_single_byte() {
        let input = &[64];
        let output = to_bytes(input);
        let expected = vec![0x40];
        assert_eq!(output, expected);
    }
    #[test]
    fn largest_single_byte() {
        let input = &[127];
        let output = to_bytes(input);
        let expected = vec![0x7f];
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_double_byte() {
        let input = &[128];
        let output = to_bytes(input);
        let expected = vec![0x81, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn arbitrary_double_byte() {
        let input = &[8_192];
        let output = to_bytes(input);
        let expected = vec![0xc0, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn largest_double_byte() {
        let input = &[16_383];
        let output = to_bytes(input);
        let expected = vec![0xff, 0x7f];
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_triple_byte() {
        let input = &[16_384];
        let output = to_bytes(input);
        let expected = vec![0x81, 0x80, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn arbitrary_triple_byte() {
        let input = &[1_048_576];
        let output = to_bytes(input);
        let expected = vec![0xc0, 0x80, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn largest_triple_byte() {
        let input = &[2_097_151];
        let output = to_bytes(input);
        let expected = vec![0xff, 0xff, 0x7f];
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_quadruple_byte() {
        let input = &[2_097_152];
        let output = to_bytes(input);
        let expected = vec![0x81, 0x80, 0x80, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn arbitrary_quadruple_byte() {
        let input = &[134_217_728];
        let output = to_bytes(input);
        let expected = vec![0xc0, 0x80, 0x80, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn largest_quadruple_byte() {
        let input = &[268_435_455];
        let output = to_bytes(input);
        let expected = vec![0xff, 0xff, 0xff, 0x7f];
        assert_eq!(output, expected);
    }
    #[test]
    fn smallest_quintuple_byte() {
        let input = &[268_435_456];
        let output = to_bytes(input);
        let expected = vec![0x81, 0x80, 0x80, 0x80, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn arbitrary_quintuple_byte() {
        let input = &[4_278_190_080];
        let output = to_bytes(input);
        let expected = vec![0x8f, 0xf8, 0x80, 0x80, 0x0];
        assert_eq!(output, expected);
    }
    #[test]
    fn maximum_32_bit_integer_input() {
        let input = &[4_294_967_295];
        let output = to_bytes(input);
        let expected = vec![0x8f, 0xff, 0xff, 0xff, 0x7f];
        assert_eq!(output, expected);
    }
    #[test]
    fn two_single_byte_values() {
        let input = &[64, 127];
        let output = to_bytes(input);
        let expected = vec![0x40, 0x7f];
        assert_eq!(output, expected);
    }
    #[test]
    fn two_multi_byte_values() {
        let input = &[16_384, 1_193_046];
        let output = to_bytes(input);
        let expected = vec![0x81, 0x80, 0x0, 0xc8, 0xe8, 0x56];
        assert_eq!(output, expected);
    }
    #[test]
    fn many_multi_byte_values() {
        let input = &[8_192, 1_193_046, 268_435_455, 0, 16_383, 16_384];
        let output = to_bytes(input);
        let expected = vec![
            0xc0, 0x0, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x0, 0xff, 0x7f, 0x81, 0x80, 0x0,
        ];
        assert_eq!(output, expected);
    }
    #[test]
    fn one_byte() {
        let input = &[0x7f];
        let output = from_bytes(input);
        let expected = Ok(vec![127]);
        assert_eq!(output, expected);
    }
    #[test]
    fn two_bytes() {
        let input = &[0xc0, 0x0];
        let output = from_bytes(input);
        let expected = Ok(vec![8_192]);
        assert_eq!(output, expected);
    }
    #[test]
    fn three_bytes() {
        let input = &[0xff, 0xff, 0x7f];
        let output = from_bytes(input);
        let expected = Ok(vec![2_097_151]);
        assert_eq!(output, expected);
    }
    #[test]
    fn four_bytes() {
        let input = &[0x81, 0x80, 0x80, 0x0];
        let output = from_bytes(input);
        let expected = Ok(vec![2_097_152]);
        assert_eq!(output, expected);
    }
    #[test]
    fn maximum_32_bit_integer() {
        let input = &[0x8f, 0xff, 0xff, 0xff, 0x7f];
        let output = from_bytes(input);
        let expected = Ok(vec![4_294_967_295]);
        assert_eq!(output, expected);
    }
    #[test]
    fn incomplete_sequence_causes_error() {
        let input = &[0xff];
        let output = from_bytes(input);
        let expected = Err(Error::IncompleteNumber);
        assert_eq!(output, expected);
    }
    #[test]
    fn incomplete_sequence_causes_error_even_if_value_is_zero() {
        let input = &[0x80];
        let output = from_bytes(input);
        let expected = Err(Error::IncompleteNumber);
        assert_eq!(output, expected);
    }
    #[test]
    fn multiple_values() {
        let input = &[
            0xc0, 0x0, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x0, 0xff, 0x7f, 0x81, 0x80, 0x0,
        ];
        let output = from_bytes(input);
        let expected = Ok(vec![8_192, 1_193_046, 268_435_455, 0, 16_383, 16_384]);
        assert_eq!(output, expected);
    }
}
