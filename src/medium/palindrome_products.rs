use std::collections::{HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64, factors: HashSet<(u64, u64)>) -> Self {
        Self { value, factors }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome = u64::MAX;
    let mut max_palindrome = u64::MIN;
    let mut min_factors: HashSet<(u64, u64)> = HashSet::new();
    let mut max_factors: HashSet<(u64, u64)> = HashSet::new();

    let update_stats = |is_new_extreme, product, current: &mut u64,
                        factors: &mut HashSet<(u64, u64)>, new_factors| {
        if is_new_extreme {
            *current = product;
            factors.clear();
            factors.insert(new_factors);
        } else if product == *current {
            factors.insert(new_factors);
        }
    };
    
    for a in min..=max {
        if a % 10 == 0 { continue };
        for b in a..=max {
            let product = a * b;
            if is_palindrome(product) {
                update_stats(product < min_palindrome, product, &mut min_palindrome, &mut min_factors, (a, b));
                update_stats(product > max_palindrome, product, &mut max_palindrome, &mut max_factors, (a, b));                
            }
        }
    }

    if min_palindrome == u64::MAX {
        None
    } else {
        Some((
            Palindrome::new(min_palindrome, min_factors),
            Palindrome::new(max_palindrome, max_factors),
        ))
    }
}

fn is_palindrome(mut n: u64) -> bool {
    if n < 10 { return true; }
    if n % 10 == 0 { return false };
    
    let mut divisor = 1;
    while n / divisor >= 10 {
        divisor *= 10;
    }

    while n > 0 {
        let first = n / divisor;
        let last = n % 10;
        if first != last {
            return false;
        }
        n = (n % divisor) / 10;
        divisor /= 100;
    }
    true
}

// better to use general version above
fn _is_palindrome(mut n: u64) -> bool {
    match n {
        0..=9 => true,
        10..=99 => n % 11 == 0,
        100..=999 => n / 100 == n % 10,
        1000..=9999 => (n / 1000 == n % 10) && ((n / 100) % 10 == (n / 10) % 10),
        10000..=99999 => (n / 10000 == n % 10) && ((n / 1000) % 10 == (n / 10) % 10),
        _ => {
            let original = n;
            let mut reversed = 0;
            while n > 0 {
                reversed = reversed * 10 + n % 10;
                n /= 10;
            }
            original == reversed
        }
    }
}

// very slow
fn __is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn find_the_smallest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 1);
        assert_eq!(pal.into_factors(), HashSet::from([(1, 1)]));
    }

    #[test]
    fn find_the_largest_palindrome_from_single_digit_factors() {
        let output = palindrome_products(1, 9);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 9);
        assert_eq!(pal.into_factors(), HashSet::from([(1, 9), (3, 3)]));
    }

    #[test]
    fn find_the_smallest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 121);
        assert_eq!(pal.into_factors(), HashSet::from([(11, 11)]));
    }

    #[test]
    fn find_the_largest_palindrome_from_double_digit_factors() {
        let output = palindrome_products(10, 99);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 9009);
        assert_eq!(pal.into_factors(), HashSet::from([(91, 99)]));
    }

    #[test]
    fn find_the_smallest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 10201);
        assert_eq!(pal.into_factors(), HashSet::from([(101, 101)]));
    }

    #[test]
    fn find_the_largest_palindrome_from_triple_digit_factors() {
        let output = palindrome_products(100, 999);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 906609);
        assert_eq!(pal.into_factors(), HashSet::from([(913, 993)]));
    }

    #[test]
    fn find_the_smallest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 1002001);
        assert_eq!(pal.into_factors(), HashSet::from([(1001, 1001)]));
    }

    #[test]
    fn find_the_largest_palindrome_from_four_digit_factors() {
        let output = palindrome_products(1000, 9999);
        assert!(output.is_some());
        let (_, pal) = output.unwrap();
        assert_eq!(pal.value(), 99000099);
        assert_eq!(pal.into_factors(), HashSet::from([(9901, 9999)]));
    }

    #[test]
    fn empty_result_for_smallest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(1002, 1003);
        assert!(output.is_none());
    }

    #[test]
    fn empty_result_for_largest_if_no_palindrome_in_the_range() {
        let output = palindrome_products(15, 15);
        assert!(output.is_none());
    }

    #[test]
    fn error_result_for_smallest_if_min_is_more_than_max() {
        let output = palindrome_products(10000, 1);
        assert!(output.is_none());
    }

    #[test]
    fn error_result_for_largest_if_min_is_more_than_max() {
        let output = palindrome_products(2, 1);
        assert!(output.is_none());
    }

    #[test]
    fn smallest_product_does_not_use_the_smallest_factor() {
        let output = palindrome_products(3215, 4000);
        assert!(output.is_some());
        let (pal, _) = output.unwrap();
        assert_eq!(pal.value(), 10988901);
        assert_eq!(pal.into_factors(), HashSet::from([(3297, 3333)]));
    }
}
