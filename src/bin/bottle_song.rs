const VERSES: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    ((start_bottles - take_down + 1)..=start_bottles)
        .rev()
        .fold(vec![], |mut res, curr| {
            res.push(verse(curr));
            res
        })
        .join("\n\n")
}

fn verse(n: u32) -> String {
    let line1 = format!(
        "{} green bottle{} hanging on the wall,",
        VERSES[n as usize],
        tails(n)
    );
    let line2 = "And if one green bottle should accidentally fall,".to_string();
    let line3 = format!(
        "There'll be {} green bottle{} hanging on the wall.",
        (VERSES[(n - 1) as usize]).to_lowercase(),
        tails(n - 1)
    );

    [line1.clone(), line1, line2, line3].join("\n")
}

fn tails(n: u32) -> String {
    match n {
        1 => "".into(),
        _ => "s".into(),
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn first_generic_verse() {
        assert_eq!(
            recite(10, 1).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn last_generic_verse() {
        assert_eq!(
            recite(3, 1).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn verse_with_2_bottles() {
        assert_eq!(
            recite(2, 1).trim(),
            concat!(
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.",
            )
        );
    }
    #[test]
    fn verse_with_1_bottle() {
        assert_eq!(
            recite(1, 1).trim(),
            concat!(
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn first_two_verses() {
        assert_eq!(
            recite(10, 2).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn last_three_verses() {
        assert_eq!(
            recite(3, 3).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn all_verses() {
        assert_eq!(
            recite(10, 10).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.\n",
                "\n",
                "Eight green bottles hanging on the wall,\n",
                "Eight green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be seven green bottles hanging on the wall.\n",
                "\n",
                "Seven green bottles hanging on the wall,\n",
                "Seven green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be six green bottles hanging on the wall.\n",
                "\n",
                "Six green bottles hanging on the wall,\n",
                "Six green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be five green bottles hanging on the wall.\n",
                "\n",
                "Five green bottles hanging on the wall,\n",
                "Five green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be four green bottles hanging on the wall.\n",
                "\n",
                "Four green bottles hanging on the wall,\n",
                "Four green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be three green bottles hanging on the wall.\n",
                "\n",
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
}
