#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(cap_1: u8, cap_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    // 3 0 3 1
    // 0 3 3 5
    // One 0 3 0 2 2 3 0 3
    // Two 5 2 2 0 5 4 4 1
    let (cap, mut state, backs, mut moves) = match start_bucket {
        Bucket::One => ([cap_1, cap_2], [cap_1, 0], [Bucket::One, Bucket::Two], 1),
        Bucket::Two => ([cap_2, cap_1], [cap_2, 0], [Bucket::Two, Bucket::One], 1),
    };

    while state.iter().all(|v| *v != goal) {
        match state {
            [_, _] if cap[1] == goal => state[1] = goal,
            [0, _] => state[0] = cap[0],
            [c0, c1] if c0 < cap[0] && c1 == cap[1] => state[1] = 0,
            [c0, c1] if c0 <= cap[0] && c1 < cap[1] => {
                state[0] = c0 - c0.min(cap[1] - c1);
                state[1] = cap[1].min(c0 + c1)
            }
            _ => return None,
        }
        moves += 1;
    }

    let (goal_bucket, other_bucket) = match state {
        [_, c] if c == goal => (backs[1], state[0]),
        [_, _] => (backs[0], state[1]),
    };

    Some(BucketStats {
        moves,
        other_bucket,
        goal_bucket,
    })
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_one() {
        // 3 0 3 1
        // 0 3 3 5
        let output = solve(3, 5, 1, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 4,
            goal_bucket: Bucket::One,
            other_bucket: 5,
        });
        assert_eq!(output, expected);
    }
    #[test]
    fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_two() {
        // One 0 3 0 2 2 3 0 3
        // Two 5 2 2 0 5 4 4 1
        let output = solve(3, 5, 1, &Bucket::Two);
        let expected = Some(BucketStats {
            moves: 8,
            goal_bucket: Bucket::Two,
            other_bucket: 3,
        });
        assert_eq!(output, expected);
    }
    #[test]
    fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_one() {
        let output = solve(7, 11, 2, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 14,
            goal_bucket: Bucket::One,
            other_bucket: 11,
        });
        assert_eq!(output, expected);
    }
    #[test]
    fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_two() {
        let output = solve(7, 11, 2, &Bucket::Two);
        let expected = Some(BucketStats {
            moves: 18,
            goal_bucket: Bucket::Two,
            other_bucket: 7,
        });
        assert_eq!(output, expected);
    }
    #[test]
    fn measure_one_step_using_bucket_one_of_size_1_and_bucket_two_of_size_3_start_with_bucket_two()
    {
        let output = solve(1, 3, 3, &Bucket::Two);
        let expected = Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        });
        assert_eq!(output, expected);
    }
    #[test]
    fn measure_using_bucket_one_of_size_2_and_bucket_two_of_size_3_start_with_bucket_one_and_end_with_bucket_two()
     {
        let output = solve(2, 3, 3, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: 2,
        });
        assert_eq!(output, expected);
    }
    #[test]
    fn not_possible_to_reach_the_goal() {
        let output = solve(6, 15, 5, &Bucket::One);
        let expected = None;
        assert_eq!(output, expected);
    }
    #[test]
    fn with_the_same_buckets_but_a_different_goal_then_it_is_possible() {
        let output = solve(6, 15, 9, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 10,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        });
        assert_eq!(output, expected);
    }
    #[test]
    fn goal_larger_than_both_buckets_is_impossible() {
        let output = solve(5, 7, 8, &Bucket::One);
        let expected = None;
        assert_eq!(output, expected);
    }
}
