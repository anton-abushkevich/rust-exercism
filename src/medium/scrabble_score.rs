/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars().map(|c| {
        match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_letter() {
        let input = "a";
        let output = score(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn uppercase_letter() {
        let input = "A";
        let output = score(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn valuable_letter() {
        let input = "f";
        let output = score(input);
        let expected = 4;
        assert_eq!(output, expected);
    }

    #[test]
    fn short_word() {
        let input = "at";
        let output = score(input);
        let expected = 2;
        assert_eq!(output, expected);
    }

    #[test]
    fn short_valuable_word() {
        let input = "zoo";
        let output = score(input);
        let expected = 12;
        assert_eq!(output, expected);
    }

    #[test]
    fn medium_word() {
        let input = "street";
        let output = score(input);
        let expected = 6;
        assert_eq!(output, expected);
    }

    #[test]
    fn medium_valuable_word() {
        let input = "quirky";
        let output = score(input);
        let expected = 22;
        assert_eq!(output, expected);
    }

    #[test]
    fn long_mixed_case_word() {
        let input = "OxyphenButazone";
        let output = score(input);
        let expected = 41;
        assert_eq!(output, expected);
    }

    #[test]
    fn english_like_word() {
        let input = "pinata";
        let output = score(input);
        let expected = 8;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_input() {
        let input = "";
        let output = score(input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn entire_alphabet_available() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = score(input);
        let expected = 87;
        assert_eq!(output, expected);
    }

    #[test]
    fn non_english_scrabble_letters_do_not_score() {
        let input = "piñata";
        let output = score(input);
        let expected = 7;
        assert_eq!(output, expected);
    }

    #[test]
    fn german_letters_do_not_score() {
        let input = "STRAßE";
        let output = score(input);
        let expected = 5;
        assert_eq!(output, expected);
    }
}