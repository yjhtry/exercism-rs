use std::{
    collections::HashMap,
    thread::{self, ScopedJoinHandle},
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut output = HashMap::new();
    thread::scope(|s| {
        let handles: Vec<ScopedJoinHandle<'_, HashMap<char, usize>>> = input
            .chunks((input.len() / worker_count).max(1))
            .map(|v| {
                s.spawn(|| {
                    v.iter()
                        .flat_map(|s| s.chars().filter(|c| c.is_alphabetic()))
                        .fold(HashMap::new(), |mut h, c| {
                            *h.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                            h
                        })
                })
            })
            .collect();

        for handle in handles {
            let h = handle.join().unwrap();

            for (k, v) in h {
                output.entry(k).and_modify(|e| *e += v).or_insert(v);
            }
        }
    });

    output
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    // Poem by Friedrich Schiller. The corresponding music is the European Anthem.
    const ODE_AN_DIE_FREUDE: [&str; 8] = [
        "Freude schöner Götterfunken",
        "Tochter aus Elysium,",
        "Wir betreten feuertrunken,",
        "Himmlische, dein Heiligtum!",
        "Deine Zauber binden wieder",
        "Was die Mode streng geteilt;",
        "Alle Menschen werden Brüder,",
        "Wo dein sanfter Flügel weilt.",
    ];
    // Dutch national anthem
    const WILHELMUS: [&str; 8] = [
        "Wilhelmus van Nassouwe",
        "ben ik, van Duitsen bloed,",
        "den vaderland getrouwe",
        "blijf ik tot in den dood.",
        "Een Prinse van Oranje",
        "ben ik, vrij, onverveerd,",
        "den Koning van Hispanje",
        "heb ik altijd geëerd.",
    ];
    // American national anthem
    const STAR_SPANGLED_BANNER: [&str; 8] = [
        "O say can you see by the dawn's early light,",
        "What so proudly we hailed at the twilight's last gleaming,",
        "Whose broad stripes and bright stars through the perilous fight,",
        "O'er the ramparts we watched, were so gallantly streaming?",
        "And the rockets' red glare, the bombs bursting in air,",
        "Gave proof through the night that our flag was still there;",
        "O say does that star-spangled banner yet wave,",
        "O'er the land of the free and the home of the brave?",
    ];
    #[test]
    fn no_texts() {
        assert_eq!(frequency(&[], 4), HashMap::new());
    }
    #[test]
    fn one_letter() {
        let mut hm = HashMap::new();
        hm.insert('a', 1);
        assert_eq!(frequency(&["a"], 4), hm);
    }
    #[test]
    fn case_insensitivity() {
        let mut hm = HashMap::new();
        hm.insert('a', 2);
        assert_eq!(frequency(&["aA"], 4), hm);
    }
    #[test]
    fn many_empty_lines() {
        let v = vec![""; 1000];
        assert_eq!(frequency(&v[..], 4), HashMap::new());
    }
    #[test]
    fn many_times_same_text() {
        let v = vec!["abc"; 1000];
        let mut hm = HashMap::new();
        hm.insert('a', 1000);
        hm.insert('b', 1000);
        hm.insert('c', 1000);
        assert_eq!(frequency(&v[..], 4), hm);
    }
    #[test]
    fn punctuation_doesnt_count() {
        assert!(!frequency(&WILHELMUS, 4).contains_key(&','));
    }
    #[test]
    fn numbers_dont_count() {
        assert!(!frequency(&["Testing, 1, 2, 3"], 4).contains_key(&'1'));
    }
    #[test]
    fn all_three_anthems_1_worker() {
        let mut v = Vec::new();
        for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
            for line in anthem.iter() {
                v.push(*line);
            }
        }
        let freqs = frequency(&v[..], 1);
        assert_eq!(freqs.get(&'a'), Some(&49));
        assert_eq!(freqs.get(&'t'), Some(&56));
        assert_eq!(freqs.get(&'ü'), Some(&2));
    }
    #[test]
    fn all_three_anthems_3_workers() {
        let mut v = Vec::new();
        for anthem in [ODE_AN_DIE_FREUDE, WILHELMUS, STAR_SPANGLED_BANNER].iter() {
            for line in anthem.iter() {
                v.push(*line);
            }
        }
        let freqs = frequency(&v[..], 3);
        assert_eq!(freqs.get(&'a'), Some(&49));
        assert_eq!(freqs.get(&'t'), Some(&56));
        assert_eq!(freqs.get(&'ü'), Some(&2));
    }
    #[test]
    fn non_integer_multiple_of_threads() {
        let v = vec!["abc"; 999];
        let mut hm = HashMap::new();
        hm.insert('a', 999);
        hm.insert('b', 999);
        hm.insert('c', 999);
        assert_eq!(frequency(&v[..], 4), hm);
    }
}
