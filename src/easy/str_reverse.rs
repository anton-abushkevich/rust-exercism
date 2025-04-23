use unicode_segmentation::UnicodeSegmentation;

pub fn reverse_with_graphemes(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(reverse("123"), "321");
        assert_eq!(reverse("Привет"), "тевирП");
        assert_eq!(reverse_with_graphemes("uüu"), "uüu");
    }
}