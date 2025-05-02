/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    calc_mmi(a, M)?;

    let mut result = String::new();
    let mut count = 0;

    for c in plaintext.chars() {
        if let Some(encoded) = match c {
            'a'..='z' | 'A'..='Z' => {
                let i = c.to_ascii_lowercase() as i32 - 'a' as i32;
                let e = (a * i + b).rem_euclid(M) as u8 + b'a';
                Some(e as char)
            }
            '0'..='9' => Some(c),
            _ => None,
        } {
            if count > 0 && count % 5 == 0 {
                result.push(' ');
            }
            result.push(encoded);
            count += 1;
        }
    }

    Ok(result)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let a_mmi = calc_mmi(a, M)?;

    Ok(ciphertext
        .chars()
        .filter_map(|c| match c {
            'a'..='z' => {
                let y = c as i32 - 'a' as i32;
                let d = (a_mmi * (y - b)).rem_euclid(M) as u8 + b'a';
                Some(d as char)
            }
            '0'..='9' => Some(c),
            _ => None,
        })
        .collect())
}

fn calc_mmi(a: i32, m: i32) -> Result<i32, AffineCipherError> {
    (1..=m).find(|&i| (a * i) % m == 1).ok_or(AffineCipherError::NotCoprime(a))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::AffineCipherError::NotCoprime;

    #[test]
    fn encode_yes() {
        let phrase = "yes";
        let (a, b) = (5, 7);
        let output = encode(phrase, a, b);
        let expected = Ok("xbt".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_no() {
        let phrase = "no";
        let (a, b) = (15, 18);
        let output = encode(phrase, a, b);
        let expected = Ok("fu".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_omg() {
        let phrase = "OMG";
        let (a, b) = (21, 3);
        let output = encode(phrase, a, b);
        let expected = Ok("lvz".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_o_m_g() {
        let phrase = "O M G";
        let (a, b) = (25, 47);
        let output = encode(phrase, a, b);
        let expected = Ok("hjp".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_mindblowingly() {
        let phrase = "mindblowingly";
        let (a, b) = (11, 15);
        let output = encode(phrase, a, b);
        let expected = Ok("rzcwa gnxzc dgt".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_numbers() {
        let phrase = "Testing,1 2 3, testing.";
        let (a, b) = (3, 4);
        let output = encode(phrase, a, b);
        let expected = Ok("jqgjc rw123 jqgjc rw".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_deep_thought() {
        let phrase = "Truth is fiction.";
        let (a, b) = (5, 17);
        let output = encode(phrase, a, b);
        let expected = Ok("iynia fdqfb ifje".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_all_the_letters() {
        let phrase = "The quick brown fox jumps over the lazy dog.";
        let (a, b) = (17, 33);
        let output = encode(phrase, a, b);
        let expected = Ok("swxtj npvyk lruol iejdc blaxk swxmh qzglf".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_with_a_not_coprime_to_m() {
        let phrase = "This is a test.";
        let (a, b) = (6, 17);
        let output = encode(phrase, a, b);
        let expected = Err(NotCoprime(6));
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_exercism() {
        let phrase = "tytgn fjr";
        let (a, b) = (3, 7);
        let output = decode(phrase, a, b);
        let expected = Ok("exercism".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_a_sentence() {
        let phrase = "qdwju nqcro muwhn odqun oppmd aunwd o";
        let (a, b) = (19, 16);
        let output = decode(phrase, a, b);
        let expected = Ok("anobstacleisoftenasteppingstone".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_numbers() {
        let phrase = "odpoz ub123 odpoz ub";
        let (a, b) = (25, 7);
        let output = decode(phrase, a, b);
        let expected = Ok("testing123testing".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_all_the_letters() {
        let phrase = "swxtj npvyk lruol iejdc blaxk swxmh qzglf";
        let (a, b) = (17, 33);
        let output = decode(phrase, a, b);
        let expected = Ok("thequickbrownfoxjumpsoverthelazydog".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_with_no_spaces_in_input() {
        let phrase = "swxtjnpvyklruoliejdcblaxkswxmhqzglf";
        let (a, b) = (17, 33);
        let output = decode(phrase, a, b);
        let expected = Ok("thequickbrownfoxjumpsoverthelazydog".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_with_too_many_spaces() {
        let phrase = "vszzm    cly   yd cg    qdp";
        let (a, b) = (15, 16);
        let output = decode(phrase, a, b);
        let expected = Ok("jollygreengiant".into());
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_with_a_not_coprime_to_m() {
        let phrase = "Test";
        let (a, b) = (13, 5);
        let output = decode(phrase, a, b);
        let expected = Err(NotCoprime(13));
        assert_eq!(output, expected);
    }
}