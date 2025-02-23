const ACTIONS: [(u8, &str); 4] = [
    (1 << 0, "wink"),
    (1 << 1, "double blink"),
    (1 << 2, "close your eyes"),
    (1 << 3, "jump"),
];
struct HandShake {
    secret: u8,
    reverse: bool,
}

impl HandShake {
    fn new(secret: u8) -> Self {
        Self {
            secret,
            reverse: (secret & 1 << 4) != 0,
        }
    }

    fn actions(&self) -> Vec<&'static str> {
        let actions = ACTIONS
            .iter()
            .filter(|(s, _)| (*s & self.secret) != 0)
            .map(|(_, a)| *a);

        if self.reverse {
            actions.rev().collect()
        } else {
            actions.collect()
        }
    }
}

pub fn actions(n: u8) -> Vec<&'static str> {
    HandShake::new(n).actions()
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn wink_for_1() {
        assert_eq!(actions(1), vec!["wink"])
    }
    #[test]
    fn double_blink_for_10() {
        assert_eq!(actions(2), vec!["double blink"])
    }
    #[test]
    fn close_your_eyes_for_100() {
        assert_eq!(actions(4), vec!["close your eyes"])
    }
    #[test]
    fn jump_for_1000() {
        assert_eq!(actions(8), vec!["jump"])
    }
    #[test]
    fn combine_two_actions() {
        assert_eq!(actions(3), vec!["wink", "double blink"])
    }
    #[test]
    fn reverse_two_actions() {
        assert_eq!(actions(19), vec!["double blink", "wink"])
    }
    #[test]
    fn reversing_one_action_gives_the_same_action() {
        assert_eq!(actions(24), vec!["jump"])
    }
    #[test]
    fn reversing_no_actions_still_gives_no_actions() {
        assert_eq!(actions(16), Vec::<&'static str>::new())
    }
    #[test]
    fn all_possible_actions() {
        assert_eq!(actions(15), vec![
            "wink",
            "double blink",
            "close your eyes",
            "jump"
        ])
    }
    #[test]
    fn reverse_all_possible_actions() {
        assert_eq!(actions(31), vec![
            "jump",
            "close your eyes",
            "double blink",
            "wink"
        ])
    }
    #[test]
    fn do_nothing_for_zero() {
        assert_eq!(actions(0), Vec::<&'static str>::new())
    }
}
