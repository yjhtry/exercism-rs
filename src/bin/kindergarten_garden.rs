use std::collections::HashMap;
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram = diagram.replace("\n", "");
    let map = HashMap::from([
        ("Alice", 0),
        ("Bob", 1),
        ("Charlie", 2),
        ("David", 3),
        ("Eve", 4),
        ("Fred", 5),
        ("Ginny", 6),
        ("Harriet", 7),
        ("Ileana", 8),
        ("Joseph", 9),
        ("Kincaid", 10),
        ("Larry", 11),
    ]);

    let idx = map.get(student).unwrap() * 2;
    let middle = diagram.len() / 2 + idx;

    diagram[idx..(idx + 2)]
        .chars()
        .chain(diagram[middle..(middle + 2)].chars())
        .map(|c| match c {
            'R' => "radishes",
            'G' => "grass",
            'C' => "clover",
            'V' => "violets",
            _ => unreachable!(),
        })
        .collect()
}

fn main() {
    println!(
        "{}",
        format!("{:b}", 111).chars().filter(|&c| c == '1').count()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn garden_with_single_student() {
        let diagram = "RC
GG";
        let student = "Alice";
        let expected = vec!["radishes", "clover", "grass", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn different_garden_with_single_student() {
        let diagram = "VC
RC";
        let student = "Alice";
        let expected = vec!["violets", "clover", "radishes", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn garden_with_two_students() {
        let diagram = "VVCG
VVRC";
        let student = "Bob";
        let expected = vec!["clover", "grass", "radishes", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn second_students_garden() {
        let diagram = "VVCCGG
VVCCGG";
        let student = "Bob";
        let expected = vec!["clover", "clover", "clover", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn third_students_garden() {
        let diagram = "VVCCGG
VVCCGG";
        let student = "Charlie";
        let expected = vec!["grass", "grass", "grass", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_alice_first_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Alice";
        let expected = vec!["violets", "radishes", "violets", "radishes"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_bob_second_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Bob";
        let expected = vec!["clover", "grass", "clover", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_charlie() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Charlie";
        let expected = vec!["violets", "violets", "clover", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_david() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "David";
        let expected = vec!["radishes", "violets", "clover", "radishes"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_eve() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Eve";
        let expected = vec!["clover", "grass", "radishes", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_fred() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Fred";
        let expected = vec!["grass", "clover", "violets", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_ginny() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Ginny";
        let expected = vec!["clover", "grass", "grass", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_harriet() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Harriet";
        let expected = vec!["violets", "radishes", "radishes", "violets"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_ileana() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Ileana";
        let expected = vec!["grass", "clover", "violets", "clover"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_joseph() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Joseph";
        let expected = vec!["violets", "clover", "violets", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_kincaid_second_to_last_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Kincaid";
        let expected = vec!["grass", "clover", "clover", "grass"];
        assert_eq!(plants(diagram, student), expected);
    }
    #[test]
    fn for_larry_last_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
        let student = "Larry";
        let expected = vec!["grass", "violets", "clover", "violets"];
        assert_eq!(plants(diagram, student), expected);
    }
}
