pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    if n < 2 {
        return factors;
    }
    let mut n = n;
    let mut i = 2;
    loop {
        if n % i == 0 {
            n /= i;
            factors.push(i);
            if n == 1 {
                return factors;
            }
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(factors(1).len(), 0);
        assert_eq!(factors(2), vec![2]);
        assert_eq!(factors(3), vec![3]);
        assert_eq!(factors(60), vec![2, 2, 3, 5]);
        assert_eq!(factors(61), vec![61]);
        assert_eq!(factors(901_255), vec![5, 17, 23, 461]);
    }
}
