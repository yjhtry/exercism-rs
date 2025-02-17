use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static GLOBAL_SET: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn get_global_set() -> &'static Mutex<HashSet<String>> {
    GLOBAL_SET.get_or_init(|| Mutex::new(HashSet::new()))
}

pub struct Robot {
    name: String,
}

impl Drop for Robot {
    fn drop(&mut self) {
        get_global_set().lock().unwrap().remove(&self.name);
    }
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Self::generator_name(),
        }
    }

    fn generator_name() -> String {
        let mut names = get_global_set().lock().unwrap();
        let mut name = String::new();
        while name.is_empty() || names.contains(&name) {
            name = format!(
                "{}{}{:3}",
                rand::random_range('A'..='Z'),
                rand::random_range('A'..='Z'),
                rand::random_range(0..1000)
            )
        }

        names.insert(name.clone());

        name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generator_name()
    }
}
fn main() {
    let bot = Robot::new();

    println!("{:?}", bot.name);
}

#[cfg(test)]
mod test {
    use super::*;
    fn assert_name_matches_pattern(n: &str) {
        assert!(n.len() == 5, "name is exactly 5 characters long");
        assert!(
            n[0..2].chars().all(|c| c.is_ascii_uppercase()),
            "name starts with 2 uppercase letters"
        );
        assert!(
            n[2..].chars().all(|c| c.is_ascii_digit()),
            "name ends with 3 numbers"
        );
    }
    #[test]
    fn name_should_match_expected_pattern() {
        let r = Robot::new();
        assert_name_matches_pattern(r.name());
    }
    #[test]
    fn different_robots_have_different_names() {
        let r1 = Robot::new();
        let r2 = Robot::new();
        assert_ne!(r1.name(), r2.name(), "Robot names should be different");
    }
    #[test]
    fn many_different_robots_have_different_names() {
        use std::collections::HashSet;
        // In 3,529 random robot names, there is ~99.99% chance of a name collision
        let vec: Vec<_> = (0..3529).map(|_| Robot::new()).collect();
        let set: HashSet<_> = vec.iter().map(|robot| robot.name()).collect();
        let number_of_collisions = vec.len() - set.len();
        assert_eq!(number_of_collisions, 0);
    }
    #[test]
    fn new_name_should_match_expected_pattern() {
        let mut r = Robot::new();
        assert_name_matches_pattern(r.name());
        r.reset_name();
        assert_name_matches_pattern(r.name());
    }
    #[test]
    fn new_name_is_different_from_old_name() {
        let mut r = Robot::new();
        let n1 = r.name().to_string();
        r.reset_name();
        let n2 = r.name().to_string();
        assert_ne!(n1, n2, "Robot name should change when reset");
    }
}
