#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(cap_1: u8, cap_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    let (cap, mut state, backs, mut moves) = match start_bucket {
        Bucket::One => ([cap_1, cap_2], [cap_1, 0], [Bucket::One, Bucket::Two], 1),
        Bucket::Two => ([cap_2, cap_1], [cap_2, 0], [Bucket::Two, Bucket::One], 1),
    };
    while state.iter().all(|v| *v != goal) {
        match state {
            [_, _] if cap[1] == goal => state[1] = goal,
            [0, _] => state[0] = cap[0],
            [c1, c2] if c1 < cap[0] && c2 == cap[1] => state[1] = 0,
            [c1, c2] if c1 <= cap[0] && c2 < cap[1] => {
                state[0] = c1 - c1.min(cap[1] - c2);
                state[1] = c2 + c1.min(cap[1] - c2);
            }
            _ => return None,
        }
        moves += 1;
    }
    let (goal_bucket, other_bucket) = match state {
        [_, c] if c == goal => (backs[1], state[0]),
        _ => (backs[0], state[1]),
    };
    Some(BucketStats {
        goal_bucket,
        other_bucket,
        moves,
    })
}
fn main() {}
