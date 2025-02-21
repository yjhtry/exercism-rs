pub fn number(user_number: &str) -> Option<String> {
    let mut clean: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();

    if let Some(stripped) = clean.strip_prefix('1') {
        clean = stripped.to_string();
    }

    if clean.len() != 10
        || clean.chars().next().unwrap() < '2'
        || clean.chars().nth(3).unwrap() < '2'
    {
        return None;
    }

    Some(clean.to_string())
}
fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cleans_the_number() {
        let input = "(223) 456-7890";
        let output = number(input);
        let expected = Some("2234567890".into());
        assert_eq!(output, expected);
    }
    #[test]
    fn cleans_numbers_with_dots() {
        let input = "223.456.7890";
        let output = number(input);
        let expected = Some("2234567890".into());
        assert_eq!(output, expected);
    }
    #[test]
    fn cleans_numbers_with_multiple_spaces() {
        let input = "223 456   7890   ";
        let output = number(input);
        let expected = Some("2234567890".into());
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_when_9_digits() {
        let input = "123456789";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_when_11_digits_does_not_start_with_a_1() {
        let input = "22234567890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn valid_when_11_digits_and_starting_with_1() {
        let input = "12234567890";
        let output = number(input);
        let expected = Some("2234567890".into());
        assert_eq!(output, expected);
    }
    #[test]
    fn valid_when_11_digits_and_starting_with_1_even_with_punctuation() {
        let input = "+1 (223) 456-7890";
        let output = number(input);
        let expected = Some("2234567890".into());
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_when_more_than_11_digits() {
        let input = "321234567890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_with_letters() {
        let input = "523-abc-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_with_punctuations() {
        let input = "523-@:!-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_area_code_starts_with_0() {
        let input = "(023) 456-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_area_code_starts_with_1() {
        let input = "(123) 456-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_exchange_code_starts_with_0() {
        let input = "(223) 056-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_exchange_code_starts_with_1() {
        let input = "(223) 156-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_area_code_starts_with_0_on_valid_11_digit_number() {
        let input = "1 (023) 456-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_area_code_starts_with_1_on_valid_11_digit_number() {
        let input = "1 (123) 456-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_exchange_code_starts_with_0_on_valid_11_digit_number() {
        let input = "1 (223) 056-7890";
        let output = number(input);
        assert!(output.is_none());
    }
    #[test]
    fn invalid_if_exchange_code_starts_with_1_on_valid_11_digit_number() {
        let input = "1 (223) 156-7890";
        let output = number(input);
        assert!(output.is_none());
    }
}
