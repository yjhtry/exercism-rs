pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        text.chars()
            .zip(indexes(self.0))
            .fold(vec![String::new(); self.0], |mut res, (c, idx)| {
                res[idx].push(c);
                res
            })
            .join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut row_indexes: Vec<_> = indexes(self.0).zip(0..).take(cipher.len()).collect();
        row_indexes.sort();
        let mut char_with_index: Vec<_> = cipher
            .chars()
            .zip(row_indexes)
            .map(|(c, (_, i))| (i, c))
            .collect();
        char_with_index.sort();
        char_with_index.into_iter().map(|(_, c)| c).collect()
    }
}

fn indexes(n: usize) -> impl Iterator<Item = usize> {
    (0..n).chain((1..n - 1).rev()).cycle()
}
fn main() {
    let mut indexesdd: Vec<_> = indexes(4).zip(1..).take(10).collect();
    indexesdd.sort();

    println!("{:?}", indexesdd);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn encode_with_two_rails() {
        //x.x.
        //.o.o
        let input = "XOXOXOXOXOXOXOXOXO";
        let rails = 2;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.encode(input);
        let expected = "XXXXXXXXXOOOOOOOOO";
        assert_eq!(output, expected);
    }
    #[test]
    fn encode_with_three_rails() {
        let input = "WEAREDISCOVEREDFLEEATONCE";
        let rails = 3;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.encode(input);
        let expected = "WECRLTEERDSOEEFEAOCAIVDEN";
        assert_eq!(output, expected);
    }
    #[test]
    fn encode_with_ending_in_the_middle() {
        // 0, 1, 2, 3, 4, 5
        // 0, 1, 2, 3, 2, 1, 0
        //e.....s..
        //.x...i.e.
        //..e.c...s
        //...r.....
        // 2
        //es  0
        //xie 1
        //ecs 2
        //r   3

        let input = "EXERCISES";
        let rails = 4;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.encode(input);
        let expected = "ESXIEECSR";
        assert_eq!(output, expected);
    }
    #[test]
    fn decode_with_three_rails() {
        let input = "TEITELHDVLSNHDTISEIIEA";
        let rails = 3;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.decode(input);
        let expected = "THEDEVILISINTHEDETAILS";
        assert_eq!(output, expected);
    }
    #[test]
    fn decode_with_five_rails() {
        let input = "EIEXMSMESAORIWSCE";
        let rails = 5;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.decode(input);
        let expected = "EXERCISMISAWESOME";
        assert_eq!(output, expected);
    }
    #[test]
    fn decode_with_six_rails() {
        let input = "133714114238148966225439541018335470986172518171757571896261";
        let rails = 6;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.decode(input);
        let expected = "112358132134558914423337761098715972584418167651094617711286";
        assert_eq!(output, expected);
    }
    #[test]
    fn encode_wide_characters() {
        let input = "古池蛙飛び込む水の音";
        let rails = 3;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.encode(input);
        let expected = "古びの池飛込水音蛙む";
        assert_eq!(output, expected);
    }
}
