pub struct RailFence(usize);

/*
// my original attempt. fails to decode wide-char cipher
impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut buckets: Vec<String> = vec![String::new(); self.0];

        let mut rail_undex = 0;
        let mut dir: i8 = 1;
        for c in text.chars() {
            buckets[rail_undex].push(c);
            rail_undex = (rail_undex as i8 + dir) as usize;
            if rail_undex == self.0 - 1 || rail_undex == 0 {
                dir *= -1;
            }
        }

        buckets.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut res: String = String::new();
        let mut buckets: Vec<String> = vec![String::new(); self.0];
        let pattern_len = 2 * self.0 - 2;

        let mut stopped_at = 0;
        for i in 0..self.0 {
            let chars_count;
            if i == 0 {
                chars_count = (cipher.len() as f32 / pattern_len as f32).ceil() as usize;
            } else if i < self.0 - 1 {
                chars_count = cipher.len() * 2 / pattern_len;
            } else {
                chars_count = cipher.len() / pattern_len;
            }
            buckets[i].push_str(&cipher[stopped_at..(stopped_at + chars_count)]);
            stopped_at += chars_count;
        }

        let mut rail_undex = 0;
        let mut dir: i8 = 1;
        for _ in cipher.chars() {
            res.push(buckets[rail_undex].drain(..1).next().unwrap());
            rail_undex = (rail_undex as i8 + dir) as usize;
            if rail_undex == self.0 - 1 || rail_undex == 0 {
                dir *= -1;
            }
        }

        res
    }
}*/

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails as usize)
    }

    fn encode_vec<T: Copy>(&self, v: Vec<T>) -> Vec<T> {
        let pattern_len = 2 * self.0 - 2;
        (0..self.0)
            .flat_map(|rail| {
                v.iter()
                    .enumerate()
                    .filter(move |(i, _)| {
                        i % pattern_len == rail || pattern_len - i % pattern_len == rail
                    })
                    .map(|(_, c)| *c)
            })
            .collect()
    }

    pub fn encode(&self, text: &str) -> String {
        let vec_chars: Vec<char> = text.chars().collect();
        self.encode_vec(vec_chars).iter().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let chars: Vec<char> = cipher.chars().collect();
        let indices: Vec<usize> = (0..chars.len()).collect();
        let mapping = self.encode_vec(indices);

        let mut char_positions: Vec<(usize, char)> = mapping.into_iter().zip(chars).collect();

        char_positions.sort_unstable_by_key(|&(pos, _)| pos);
        char_positions.into_iter().map(|(_, c)| c).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_with_two_rails() {
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

    #[test]
    fn decode_wide_characters() {
        let input = "古びの池飛込水音蛙む";
        let rails = 3;
        let rail_fence = RailFence::new(rails);
        let output = rail_fence.decode(input);
        let expected = "古池蛙飛び込む水の音";
        assert_eq!(output, expected);
    }
}
