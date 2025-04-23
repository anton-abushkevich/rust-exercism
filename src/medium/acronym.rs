pub fn _abbreviate(phrase: &str) -> String {
    let mut acromym = String::new();
    let mut prev = ' ';
    for c in phrase.chars() {
        if prev == ' ' && c != '-' && c != '_' ||
            prev == '_' && c != ' ' ||
            prev == '-' && c != ' ' ||
            prev.is_ascii_lowercase() && c.is_ascii_uppercase() {
            acromym.push(c.to_ascii_uppercase());
        }
        prev = c
    }
    acromym
}

pub fn abbreviate(phrase: &str) -> String {
    let mut acromym = String::new();
    let mut prev = ' ';
    for c in phrase.chars() {
        if c.is_alphabetic() &&
            (prev == ' ' || prev == '_' || prev == '-' ||
                prev.is_ascii_lowercase() && c.is_ascii_uppercase()) {
                acromym.push(c.to_ascii_uppercase());
        }
        prev = c
    }
    acromym
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_underscores() {
        assert_eq!(abbreviate("_I am human_"), "IAH");
    }

    #[test]
    fn basic() {
        let input = "Portable Network Graphics";
        let output = abbreviate(input);
        let expected = "PNG";
        assert_eq!(output, expected);
    }

    #[test]
    fn lowercase_words() {
        let input = "Ruby on Rails";
        let output = abbreviate(input);
        let expected = "ROR";
        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation() {
        let input = "First In, First Out";
        let output = abbreviate(input);
        let expected = "FIFO";
        assert_eq!(output, expected);
    }

    #[test]
    fn all_caps_word() {
        let input = "GNU Image Manipulation Program";
        let output = abbreviate(input);
        let expected = "GIMP";
        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation_without_whitespace() {
        let input = "Complementary metal-oxide semiconductor";
        let output = abbreviate(input);
        let expected = "CMOS";
        assert_eq!(output, expected);
    }

    #[test]
    fn very_long_abbreviation() {
        let input = "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me";
        let output = abbreviate(input);
        let expected = "ROTFLSHTMDCOALM";
        assert_eq!(output, expected);
    }

    #[test]
    fn consecutive_delimiters() {
        let input = "Something - I made up from thin air";
        let output = abbreviate(input);
        let expected = "SIMUFTA";
        assert_eq!(output, expected);
    }

    #[test]
    fn apostrophes() {
        let input = "Halley's Comet";
        let output = abbreviate(input);
        let expected = "HC";
        assert_eq!(output, expected);
    }

    #[test]
    fn underscore_emphasis() {
        let input = "The Road _Not_ Taken";
        let output = abbreviate(input);
        let expected = "TRNT";
        assert_eq!(output, expected);
    }

    #[test]
    fn camelcase() {
        let input = "HyperText Markup Language";
        let output = abbreviate(input);
        let expected = "HTML";
        assert_eq!(output, expected);
    }
}
