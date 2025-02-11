#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

const DNA: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA: [char; 4] = ['G', 'C', 'A', 'U'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|c| !DNA.contains(&c)) {
            Some(x) => Err(x),
            None => Ok(Self(dna.to_string())),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .bytes()
            .map(|c| match c {
                b'G' => 'C',
                b'C' => 'G',
                b'T' => 'A',
                b'A' => 'U',
                _ => unreachable!(),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|c| !RNA.contains(&c)) {
            Some(x) => Err(x),
            None => Ok(Self(rna.to_string())),
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn empty_rna_sequence() {
        let input = "";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_cytosine_is_guanine() {
        let input = "C";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("G").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_guanine_is_cytosine() {
        let input = "G";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("C").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_thymine_is_adenine() {
        let input = "T";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("A").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement_of_adenine_is_uracil() {
        let input = "A";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("U").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn rna_complement() {
        let input = "ACGTGGTCTTAA";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("UGCACCAGAAUU").unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_dna_input() {
        let input = "U";
        let output = Dna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_dna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Dna::new(input);
        let expected = Err(4);
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_rna_input() {
        let input = "T";
        let output = Rna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }
    #[test]
    fn invalid_rna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Rna::new(input);
        let expected = Err(3);
        assert_eq!(output, expected);
    }
}
