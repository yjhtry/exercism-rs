use std::collections::HashMap;

const NUCLEOTIDES: &str = "ACGT";
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    NUCLEOTIDES
        .contains(nucleotide)
        .then_some(dna.chars().try_fold(0_usize, |sum, c| {
            NUCLEOTIDES
                .contains(c)
                .then_some(if nucleotide == c { sum + 1 } else { sum })
                .ok_or(c)
        }))
        .ok_or(nucleotide)?
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDES.chars().try_fold(HashMap::new(), |mut h, c| {
        count(c, dna).map(|count| {
            h.insert(c, count);
            h
        })
    })
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn count_empty() {
        assert_eq!(count('A', ""), Ok(0));
    }
    #[test]
    fn count_invalid_nucleotide() {
        assert_eq!(count('X', "A"), Err('X'));
    }
    #[test]
    fn count_invalid_dna() {
        assert_eq!(count('A', "AX"), Err('X'));
    }
    #[test]
    fn count_repetitive_cytosine() {
        assert_eq!(count('C', "CCCCC"), Ok(5));
    }
    #[test]
    fn count_only_thymine() {
        assert_eq!(count('T', "GGGGGTAACCCGG"), Ok(1));
    }
    #[test]
    fn empty_strand() {
        let output = nucleotide_counts("");
        let mut expected = HashMap::new();
        expected.insert('A', 0);
        expected.insert('C', 0);
        expected.insert('G', 0);
        expected.insert('T', 0);
        assert_eq!(output, Ok(expected));
    }
    #[test]
    fn can_count_one_nucleotide_in_single_character_input() {
        let output = nucleotide_counts("G");
        let mut expected = HashMap::new();
        expected.insert('A', 0);
        expected.insert('C', 0);
        expected.insert('G', 1);
        expected.insert('T', 0);
        assert_eq!(output, Ok(expected));
    }
    #[test]
    fn strand_with_repeated_nucleotide() {
        let output = nucleotide_counts("GGGGGGG");
        let mut expected = HashMap::new();
        expected.insert('A', 0);
        expected.insert('C', 0);
        expected.insert('G', 7);
        expected.insert('T', 0);
        assert_eq!(output, Ok(expected));
    }
    #[test]
    fn strand_with_multiple_nucleotides() {
        let output = nucleotide_counts(
            "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC",
        );
        let mut expected = HashMap::new();
        expected.insert('A', 20);
        expected.insert('C', 12);
        expected.insert('G', 17);
        expected.insert('T', 21);
        assert_eq!(output, Ok(expected));
    }
    #[test]
    fn strand_with_invalid_nucleotides() {
        let output = nucleotide_counts("AGXXACT");
        assert!(output.is_err());
    }
}
