pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                match c.is_uppercase() {
                    true => char::from(b'Z' - c as u8 + b'a'),
                    false => char::from(b'z' - c as u8 + b'a'),
                }
            } else {
                c
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(String::from_iter)
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                (b'z' - c as u8 + b'a').into()
            } else {
                c
            }
        })
        .collect()
}

fn main() {}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn encode_yes() {
        assert_eq!(encode("yes"), "bvh");
    }
    #[test]
    fn encode_no() {
        assert_eq!(encode("no"), "ml");
    }
    #[test]
    fn encode_omg() {
        assert_eq!(encode("OMG"), "lnt");
    }
    #[test]
    fn encode_spaces() {
        assert_eq!(encode("O M G"), "lnt");
    }
    #[test]
    fn encode_mindblowingly() {
        assert_eq!(encode("mindblowingly"), "nrmwy oldrm tob");
    }
    #[test]
    fn encode_numbers() {
        assert_eq!(encode("Testing,1 2 3, testing."), "gvhgr mt123 gvhgr mt");
    }
    #[test]
    fn encode_deep_thought() {
        assert_eq!(encode("Truth is fiction."), "gifgs rhurx grlm");
    }
    #[test]
    fn encode_all_the_letters() {
        assert_eq!(
            encode("The quick brown fox jumps over the lazy dog."),
            "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"
        );
    }
    #[test]
    fn decode_exercism() {
        assert_eq!(decode("vcvix rhn"), "exercism");
    }
    #[test]
    fn decode_a_sentence() {
        assert_eq!(
            decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v"),
            "anobstacleisoftenasteppingstone"
        );
    }
    #[test]
    fn decode_numbers() {
        assert_eq!(decode("gvhgr mt123 gvhgr mt"), "testing123testing");
    }
    #[test]
    fn decode_all_the_letters() {
        assert_eq!(
            decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"),
            "thequickbrownfoxjumpsoverthelazydog"
        );
    }
    #[test]
    fn decode_with_too_many_spaces() {
        assert_eq!(decode("vc vix    r hn"), "exercism");
    }
    #[test]
    fn decode_with_no_spaces() {
        assert_eq!(
            decode("zmlyhgzxovrhlugvmzhgvkkrmthglmv"),
            "anobstacleisoftenasteppingstone"
        );
    }
}
