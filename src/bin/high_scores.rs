#[derive(Debug)]
pub struct HighScores(pub Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self(scores.into())
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.0.clone();
        scores.sort_by(|a, b| b.cmp(a));

        scores.truncate(3);

        scores
    }
}
fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn list_of_scores() {
        let expected = [30, 50, 20, 70];
        let high_scores = HighScores::new(&expected);
        assert_eq!(high_scores.scores(), &expected);
    }
    #[test]
    fn latest_score() {
        let high_scores = HighScores::new(&[100, 0, 90, 30]);
        assert_eq!(high_scores.latest(), Some(30));
    }
    #[test]
    fn latest_score_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.latest(), None);
    }
    #[test]
    fn personal_best() {
        let high_scores = HighScores::new(&[40, 100, 70]);
        assert_eq!(high_scores.personal_best(), Some(100));
    }
    #[test]
    fn personal_best_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.personal_best(), None);
    }
    #[test]
    fn personal_top_three() {
        let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
    }
    #[test]
    fn personal_top_three_highest_to_lowest() {
        let high_scores = HighScores::new(&[20, 10, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
    }
    #[test]
    fn personal_top_three_with_tie() {
        let high_scores = HighScores::new(&[40, 20, 40, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![40, 40, 30]);
    }
    #[test]
    fn personal_top_three_with_less_than_three_scores() {
        let high_scores = HighScores::new(&[30, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![70, 30]);
    }
    #[test]
    fn personal_top_three_only_one_score() {
        let high_scores = HighScores::new(&[40]);
        assert_eq!(high_scores.personal_top_three(), vec![40]);
    }
    #[test]
    fn personal_top_three_empty() {
        let high_scores = HighScores::new(&[]);
        assert!(high_scores.personal_top_three().is_empty());
    }
}
