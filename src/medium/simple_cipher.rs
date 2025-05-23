use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    codec(key, s, shift_fwd)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    codec(key, s, shift_bwd)
}

pub fn encode_random(s: &str) -> (String, String) {
    // why (String, String)? what about invalid s?
    if !is_valid(s) { return (String::new(), s.to_string()) }

    let mut rng = rand::rng();
    let key: String = (0..100)
        .map(|_| (rng.random_range(b'a'..=b'z') as char))
        .collect();

    let encoded = codec(key.as_str(), s, shift_fwd).unwrap();
    (key, encoded)
}

fn codec<F: Fn(char, char) -> char>(key: &str, s: &str, shift: F) -> Option<String> {
    if !is_valid(key) || !is_valid(s) { return None }

    let key: Vec<char> = key.chars().collect();
    Some(s.chars()
        .enumerate()
        .map(|(i, c)| shift(c, key[i % key.len()]))
        .collect())
}

fn shift_fwd(c: char, shift: char) -> char {
    (b'a' + (c as u8 + shift as u8 - 2*b'a') % 26) as char
}

fn shift_bwd(c: char, shift: char) -> char {
    (b'a' + (c as u8 - b'a' + 26 - (shift as u8 - b'a')) % 26) as char
}

fn is_valid(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    const PLAIN_TEXT: &str = "thisismysecret";
    const KEY: &str = "abcdefghij";

    #[test]
    fn cipher_can_encode_with_given_key() {
        assert_eq!(encode(KEY, "aaaaaaaaaa"), Some(KEY.to_string()));
    }

    #[test]
    fn cipher_can_decode_with_given_key() {
        assert_eq!(decode(KEY, "abcdefghij"), Some("aaaaaaaaaa".to_string()));
    }

    #[test]
    fn cipher_is_reversible_given_key() {
        assert_eq!(
            decode(KEY, &encode(KEY, PLAIN_TEXT).unwrap()),
            Some(PLAIN_TEXT.to_string())
        );
    }

    #[test]
    fn cipher_can_double_shift_encode() {
        let plain_text = "iamapandabear";
        assert_eq!(
            encode(plain_text, plain_text),
            Some("qayaeaagaciai".to_string())
        );
    }

    #[test]
    fn cipher_can_wrap_encode() {
        assert_eq!(encode(KEY, "zzzzzzzzzz"), Some("zabcdefghi".to_string()));
    }

    #[test]
    fn cipher_can_encode_a_message_that_is_shorter_than_the_key() {
        assert_eq!(encode(KEY, "aaaaa"), Some("abcde".to_string()));
    }

    #[test]
    fn cipher_can_decode_a_message_that_is_shorter_than_the_key() {
        assert_eq!(decode(KEY, "abcde"), Some("aaaaa".to_string()));
    }

    #[test]
    fn encode_returns_none_with_an_all_caps_key() {
        let key = "ABCDEF";
        assert_eq!(encode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn encode_returns_none_with_an_any_caps_key() {
        let key = "abcdEFg";
        assert_eq!(encode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn encode_returns_none_with_numeric_key() {
        let key = "12345";
        assert_eq!(encode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn encode_returns_none_with_any_numeric_key() {
        let key = "abcd345ef";
        assert_eq!(encode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn encode_returns_none_with_empty_key() {
        let key = "";
        assert_eq!(encode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn decode_returns_none_with_an_all_caps_key() {
        let key = "ABCDEF";
        assert_eq!(decode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn decode_returns_none_with_an_any_caps_key() {
        let key = "abcdEFg";
        assert_eq!(decode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn decode_returns_none_with_numeric_key() {
        let key = "12345";
        assert_eq!(decode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn decode_returns_none_with_any_numeric_key() {
        let key = "abcd345ef";
        assert_eq!(decode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn decode_returns_none_with_empty_key() {
        let key = "";
        assert_eq!(decode(key, PLAIN_TEXT), None);
    }

    #[test]
    fn encode_random_uses_key_made_of_letters() {
        let (k, _) = encode_random(PLAIN_TEXT);
        assert!(k.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn encode_random_uses_key_of_100_characters_or_more() {
        let (k, _) = encode_random(PLAIN_TEXT);
        assert!(k.len() >= 100);
    }

    #[test]
    fn encode_random_uses_randomly_generated_key() {
        let mut keys = HashSet::new();
        let trials = 100;
        for _ in 0..trials {
            keys.insert(encode_random(PLAIN_TEXT).0);
        }

        assert_eq!(keys.len(), trials);
    }

    #[test]
    fn encode_random_can_encode() {
        let (k, encoded) = encode_random("aaaaaaaaaa");
        assert_eq!(encoded, k.split_at(10).0);
    }

    #[test]
    fn encode_random_can_decode() {
        let (k, _) = encode_random("aaaaaaaaaa");
        assert_eq!(decode(&k, k.split_at(10).0), Some("aaaaaaaaaa".to_string()));
    }

    #[test]
    fn encode_random_is_reversible() {
        let (k, encoded) = encode_random(PLAIN_TEXT);
        assert_eq!(decode(&k, &encoded), Some(PLAIN_TEXT.to_string()));
    }
}