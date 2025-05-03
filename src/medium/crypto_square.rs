pub fn encrypt(input: &str) -> String {
    let normalized: Vec<u8> = input.bytes()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if normalized.is_empty() { return String::new() }

    let len = normalized.len();
    let c = (len as f32).sqrt().ceil() as usize;
    let r = len.div_ceil(c);
    let mut encrypted: Vec<u8> = Vec::with_capacity(c * r + r - 1); // + r-1 spaces
    
    for i in 0..c {
        for j in 0..r {
            encrypted.push(*normalized.get( j * c + i).unwrap_or(&b' '));
        }
    }
    String::from_utf8(encrypted.chunks(r).collect::<Vec<_>>().join(&b' ')).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_plaintext_results_in_an_empty_ciphertext() {
        let actual = encrypt("");
        let expected = "";
        assert_eq!(&actual, expected);
    }

    #[test]
    fn normalization_results_in_empty_plaintext() {
        let actual = encrypt("... --- ...");
        let expected = "";
        assert_eq!(&actual, expected);
    }

    #[test]
    fn lowercase() {
        let actual = encrypt("A");
        let expected = "a";
        assert_eq!(&actual, expected);
    }

    #[test]
    fn remove_spaces() {
        let actual = encrypt("  b ");
        let expected = "b";
        assert_eq!(&actual, expected);
    }

    #[test]
    fn remove_punctuation() {
        let actual = encrypt("@1,%!");
        let expected = "1";
        assert_eq!(&actual, expected);
    }

    #[test]
    fn test_9_character_plaintext_results_in_3_chunks_of_3_characters() {
        let actual = encrypt("This is fun!");
        let expected = "tsf hiu isn";
        assert_eq!(&actual, expected);
    }

    #[test]
    fn test_8_character_plaintext_results_in_3_chunks_the_last_one_with_a_trailing_space() {
        let actual = encrypt("Chill out.");
        let expected = "clu hlt io ";
        assert_eq!(&actual, expected);
    }

    #[test]
    fn test_54_character_plaintext_results_in_7_chunks_the_last_two_with_trailing_spaces() {
        let actual = encrypt("If man was meant to stay on the ground, god would have given us roots.");
        let expected = "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau ";
        assert_eq!(&actual, expected);
    }
}