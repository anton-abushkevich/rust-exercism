pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits_len = digits.len();
    let mut v: Vec<String> = Vec::new();
    if digits_len < len {
        return v
    }

    let mut i = 0;
    loop {
        let j = i + len;
        v.push(String::from(&digits[i..j]));
        if j == digits_len {
            break
        }
        i += 1;
    }
    v
}

pub fn _series_fn(digits: &str, len: usize) -> Vec<String> {
    digits.chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|c| c.iter().collect())
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        for i in &series("843216546", 4) {
            println!("{i}");
        }
    }
}