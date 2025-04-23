pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typical_leap_year() {
        assert!(is_leap_year(2004));  // Делится на 4, но не на 100
    }

    #[test]
    fn test_typical_common_year() {
        assert!(!is_leap_year(2003)); // Не делится на 4
    }

    #[test]
    fn test_atypical_common_year() {
        assert!(!is_leap_year(1900)); // Делится на 100, но не на 400
    }

    #[test]
    fn test_atypical_leap_year() {
        assert!(is_leap_year(2000));  // Делится на 400
    }
}