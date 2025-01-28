use std::collections::HashMap;

fn main() {
    pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
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

        let idx = map.get(student).unwrap();
        let middle = diagram.len() / 2 + idx;

        diagram[*idx..(*idx + 2)]
            .chars()
            .chain(diagram[middle..(middle + 2)].chars())
            .map(|c| match c {
                'G' => "Grass",
                'C' => "Clover",
                'R' => "Radish",
                'V' => "Violet",
                _ => unreachable!(),
            })
            .collect()
    }
}
