// pub fn check(candidate: &str) -> bool {
//     let s = candidate.to_uppercase().replace(['-', ' '], "");
//
//     s.len() == s.bytes().collect::<HashSet<u8>>().len()
// }

pub fn check(candidate: &str) -> bool {
    let l_a: u8 = b'a';
    let l_z: u8 = b'z';
    let u_a: u8 = b'A';
    let u_z: u8 = b'Z';

    let mut flag = 0;
    for c in candidate.bytes() {
        if c >= l_a && c <= l_z {
            if flag & (1 << (c - l_a)) != 0 {
                return false;
            } else {
                flag |= 1 << (c - l_a);
            }
        }
        if c >= u_a && c <= u_z {
            if flag & (1 << (c - u_a)) != 0 {
                return false;
            } else {
                flag |= 1 << (c - u_a);
            }
        }
    }

    true
}

fn main() {
    check("six-year");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_string() {
        assert!(check(""));
    }
    #[test]
    fn isogram_with_only_lower_case_characters() {
        assert!(check("isogram"));
    }
    #[test]
    fn word_with_one_duplicated_character() {
        assert!(!check("eleven"));
    }
    #[test]
    fn word_with_one_duplicated_character_from_the_end_of_the_alphabet() {
        assert!(!check("zzyzx"));
    }
    #[test]
    fn longest_reported_english_isogram() {
        assert!(check("subdermatoglyphic"));
    }
    #[test]
    fn word_with_duplicated_character_in_mixed_case() {
        assert!(!check("Alphabet"));
    }
    #[test]
    fn word_with_duplicated_character_in_mixed_case_lowercase_first() {
        assert!(!check("alphAbet"));
    }
    #[test]
    fn hypothetical_isogrammic_word_with_hyphen() {
        assert!(check("thumbscrew-japingly"));
    }
    #[test]
    fn hypothetical_word_with_duplicated_character_following_hyphen() {
        assert!(!check("thumbscrew-jappingly"));
    }
    #[test]
    fn isogram_with_duplicated_hyphen() {
        assert!(check("six-year-old"));
    }
    #[test]
    fn made_up_name_that_is_an_isogram() {
        assert!(check("Emily Jung Schwartzkopf"));
    }
    #[test]
    fn duplicated_character_in_the_middle() {
        assert!(!check("accentor"));
    }
    #[test]
    fn same_first_and_last_characters() {
        assert!(!check("angola"));
    }
    #[test]
    fn word_with_duplicated_character_and_with_two_hyphens() {
        assert!(!check("up-to-date"));
    }
}
