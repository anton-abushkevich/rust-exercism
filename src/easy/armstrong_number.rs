pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }

    let num_str = num.to_string();
    let len = num_str.len() as u32;
    let mut res: u32 = 0;

    for ch in num_str.chars() {
        res += ch.to_digit(10).unwrap().pow(len);
    }

    res == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(is_armstrong_number(5), true);
        assert_eq!(is_armstrong_number(15), false);
        assert_eq!(is_armstrong_number(153), true);
    }
}

// решение через итератор
pub fn _is_armstrong_number_fn(num: u32) -> bool {
    if num < 10 {
        return true;
    }

    let num_str = num.to_string();
    let len = num_str.len() as u32;

    num_str.chars().map(|ch| {
        ch.to_digit(10).unwrap().pow(len)
    }).sum::<u32>() == num
}