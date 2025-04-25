/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    (s1.len() == s2.len())
        .then_some(s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count())
}

pub fn _hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None
    }
    let mut count = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            count += 1;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strands() {
        assert_eq!(hamming_distance("", ""), Some(0));
    }

    #[test]
    fn single_letter_identical_strands() {
        assert_eq!(hamming_distance("A", "A"), Some(0));
    }

    #[test]
    fn single_letter_different_strands() {
        assert_eq!(hamming_distance("G", "T"), Some(1));
    }

    #[test]
    fn long_identical_strands() {
        assert_eq!(hamming_distance("GGACTGAAATCTG", "GGACTGAAATCTG"), Some(0));
    }

    #[test]
    fn long_different_strands() {
        assert_eq!(hamming_distance("GGACGGATTCTG", "AGGACGGATTCT"), Some(9));
    }

    #[test]
    fn disallow_first_strand_longer() {
        assert_eq!(hamming_distance("AATG", "AAA"), None);
    }

    #[test]
    fn disallow_second_strand_longer() {
        assert_eq!(hamming_distance("ATA", "AGTG"), None);
    }

    #[test]
    fn disallow_empty_first_strand() {
        assert_eq!(hamming_distance("", "G"), None);
    }

    #[test]
    fn disallow_empty_second_strand() {
        assert_eq!(hamming_distance("G", ""), None);
    }

}