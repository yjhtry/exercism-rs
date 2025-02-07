use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// The core idea is to transform equation before permutations testing, to calculate it as quickly as possible.
// The simplest data model for testing would be just a list of letter factors (coefficiens),
//     so that factors multiplied by letter values and summized will give 0 for correct solution.
// For that, we should got through the equation, sum and remember factors per letter,
//     determined by the letter position in the word (x1, x10, x100, etc).
// After "==" we should change the sign of the factors, and it's convenient to parse reversed input string.
// Sorting letters/factors also impact on performance, as I've found.
// Additionally, we have to check found solution against "no zero first" rule,
//     so we need to know which letters occurs at the first position in an encoded number.
fn calc_factors(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::new();
    let mut sign = -1;
    let mut pos = 0;
    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                sign = 1;
                pos = 0
            }
            '+' => pos = 0,
            _ => {
                *factors.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                pos += 1;
            }
        }
    }
    factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let firsts = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect::<HashSet<_>>();
    let (letters, factors) = calc_factors(input);
    for perm in (0..=9).permutations(letters.len()) {
        let sum = perm
            .iter()
            .enumerate()
            .map(|(i, v)| v * factors.get(i).unwrap())
            .sum::<i64>();
        if sum == 0
            && !perm
                .iter()
                .enumerate()
                .any(|(i, v)| *v == 0 && firsts.contains(letters.get(i).unwrap()))
        {
            return Some(HashMap::from_iter(
                perm.iter()
                    .enumerate()
                    .map(|(i, v)| (*letters.get(i).unwrap(), *v as u8)),
            ));
        }
    }
    None
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init_hash_map() {
        let answer = solve("I + BB == ILL");
        let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn puzzle_with_three_letters() {
        let answer = solve("I + BB == ILL");
        let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn solution_must_have_unique_value_for_each_letter() {
        let answer = solve("A == B");
        assert_eq!(answer, None);
    }
    #[test]
    fn leading_zero_solution_is_invalid() {
        let answer = solve("ACA + DD == BD");
        assert_eq!(answer, None);
    }
    #[test]
    fn puzzle_with_two_digits_final_carry() {
        let answer = solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC");
        let expected = [('A', 9), ('B', 1), ('C', 0)].into_iter().collect();
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn puzzle_with_four_letters() {
        let answer = solve("AS + A == MOM");
        let expected = [('A', 9), ('S', 2), ('M', 1), ('O', 0)]
            .into_iter()
            .collect();
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn puzzle_with_six_letters() {
        // left: Some({'L': 1, 'E': 4, 'A': 0, 'T': 8, 'N': 9, 'O': 8})
        // right: Some({'A': 0, 'E': 2, 'O': 4, 'T': 9, 'L': 1, 'N': 7})

        // 1097
        let answer = solve("NO + NO + TOO == LATE");
        let expected = [('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]
            .into_iter()
            .collect();
        // println!("{:?}", answer);
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn puzzle_with_seven_letters() {
        let answer = solve("HE + SEES + THE == LIGHT");
        let expected = [
            ('E', 4),
            ('G', 2),
            ('H', 5),
            ('I', 0),
            ('L', 1),
            ('S', 9),
            ('T', 7),
        ]
        .into_iter()
        .collect();
        // println!("{:?}", answer);
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn puzzle_with_eight_letters() {
        let answer = solve("SEND + MORE == MONEY");
        let expected = [
            ('S', 9),
            ('E', 5),
            ('N', 6),
            ('D', 7),
            ('M', 1),
            ('O', 0),
            ('R', 8),
            ('Y', 2),
        ]
        .into_iter()
        .collect();
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn puzzle_with_ten_letters() {
        let answer = solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE");
        let expected = [
            ('A', 5),
            ('D', 3),
            ('E', 4),
            ('F', 7),
            ('G', 8),
            ('N', 0),
            ('O', 2),
            ('R', 1),
            ('S', 6),
            ('T', 9),
        ]
        .into_iter()
        .collect();
        assert_eq!(answer, Some(expected));
    }
    #[test]
    fn puzzle_with_ten_letters_and_199_addends() {
        let answer = solve(
            "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
        );
        let expected = [
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9),
        ]
        .into_iter()
        .collect();
        assert_eq!(answer, Some(expected));
    }
}
