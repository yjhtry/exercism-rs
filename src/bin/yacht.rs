use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

trait Utils {
    fn chunk_same(&self) -> Vec<Vec<u8>>;
}

impl Utils for Dice {
    fn chunk_same(&self) -> Vec<Vec<u8>> {
        let mut h = HashMap::new();
        for n in self.iter() {
            h.entry(n).or_insert(vec![]).push(*n);
        }
        h.into_values().collect()
    }
}

pub fn score(mut dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => dice.into_iter().filter(|n| *n == 1).sum(),
        Category::Twos => dice.into_iter().filter(|n| *n == 2).sum(),
        Category::Threes => dice.into_iter().filter(|n| *n == 3).sum(),
        Category::Fours => dice.into_iter().filter(|n| *n == 4).sum(),
        Category::Fives => dice.into_iter().filter(|n| *n == 5).sum(),
        Category::Sixes => dice.into_iter().filter(|n| *n == 6).sum(),
        Category::FullHouse => {
            let chunks = dice.chunk_same();
            match chunks.len() {
                2 => match (chunks[0].len(), chunks[1].len()) {
                    (2, 3) | (3, 2) => dice.iter().sum(),
                    _ => 0,
                },
                _ => 0,
            }
        }
        Category::FourOfAKind => {
            let chunks = dice.chunk_same();
            match chunks.len() {
                1 => chunks[0][0] * 4,
                2 => match (chunks[0].len(), chunks[1].len()) {
                    (4, 1) => chunks[0][0] * 4,
                    (1, 4) => chunks[1][0] * 4,
                    _ => 0,
                },
                _ => 0,
            }
        }
        Category::LittleStraight => {
            dice.sort();
            if dice == [1, 2, 3, 4, 5] { 30 } else { 0 }
        }
        Category::BigStraight => {
            dice.sort();
            if dice == [2, 3, 4, 5, 6] { 30 } else { 0 }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => match dice.chunk_same()[0].len() {
            5 => 50,
            _ => 0,
        },
    }
}
fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn yacht() {
        let expected = 50;
        assert_eq!(score([5, 5, 5, 5, 5], Category::Yacht), expected);
    }
    #[test]
    fn not_yacht() {
        let expected = 0;
        assert_eq!(score([1, 3, 3, 2, 5], Category::Yacht), expected);
    }
    #[test]
    fn ones() {
        let expected = 3;
        assert_eq!(score([1, 1, 1, 3, 5], Category::Ones), expected);
    }
    #[test]
    fn ones_out_of_order() {
        let expected = 3;
        assert_eq!(score([3, 1, 1, 5, 1], Category::Ones), expected);
    }
    #[test]
    fn no_ones() {
        let expected = 0;
        assert_eq!(score([4, 3, 6, 5, 5], Category::Ones), expected);
    }
    #[test]
    fn twos() {
        let expected = 2;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Twos), expected);
    }
    #[test]
    fn fours() {
        let expected = 8;
        assert_eq!(score([1, 4, 1, 4, 1], Category::Fours), expected);
    }
    #[test]
    fn yacht_counted_as_threes() {
        let expected = 15;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Threes), expected);
    }
    #[test]
    fn yacht_of_3s_counted_as_fives() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Fives), expected);
    }
    #[test]
    fn fives() {
        let expected = 10;
        assert_eq!(score([1, 5, 3, 5, 3], Category::Fives), expected);
    }
    #[test]
    fn sixes() {
        let expected = 6;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Sixes), expected);
    }
    #[test]
    fn full_house_two_small_three_big() {
        let expected = 16;
        assert_eq!(score([2, 2, 4, 4, 4], Category::FullHouse), expected);
    }
    #[test]
    fn full_house_three_small_two_big() {
        let expected = 19;
        assert_eq!(score([5, 3, 3, 5, 3], Category::FullHouse), expected);
    }
    #[test]
    fn two_pair_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 4, 4, 5], Category::FullHouse), expected);
    }
    #[test]
    fn four_of_a_kind_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([1, 4, 4, 4, 4], Category::FullHouse), expected);
    }
    #[test]
    fn yacht_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 2, 2, 2], Category::FullHouse), expected);
    }
    #[test]
    fn four_of_a_kind() {
        let expected = 24;
        assert_eq!(score([6, 6, 4, 6, 6], Category::FourOfAKind), expected);
    }
    #[test]
    fn yacht_can_be_scored_as_four_of_a_kind() {
        let expected = 12;
        assert_eq!(score([3, 3, 3, 3, 3], Category::FourOfAKind), expected);
    }
    #[test]
    fn full_house_is_not_four_of_a_kind() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 5, 5], Category::FourOfAKind), expected);
    }
    #[test]
    fn little_straight() {
        let expected = 30;
        assert_eq!(score([3, 5, 4, 1, 2], Category::LittleStraight), expected);
    }
    #[test]
    fn little_straight_as_big_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 5], Category::BigStraight), expected);
    }
    #[test]
    fn four_in_order_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 2, 3, 4], Category::LittleStraight), expected);
    }
    #[test]
    fn no_pairs_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 6], Category::LittleStraight), expected);
    }
    #[test]
    fn minimum_is_1_maximum_is_5_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 3, 4, 5], Category::LittleStraight), expected);
    }
    #[test]
    fn big_straight() {
        let expected = 30;
        assert_eq!(score([4, 6, 2, 5, 3], Category::BigStraight), expected);
    }
    #[test]
    fn big_straight_as_little_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 2], Category::LittleStraight), expected);
    }
    #[test]
    fn no_pairs_but_not_a_big_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 1], Category::BigStraight), expected);
    }
    #[test]
    fn choice() {
        let expected = 23;
        assert_eq!(score([3, 3, 5, 6, 6], Category::Choice), expected);
    }
    #[test]
    fn yacht_as_choice() {
        let expected = 10;
        assert_eq!(score([2, 2, 2, 2, 2], Category::Choice), expected);
    }
}
