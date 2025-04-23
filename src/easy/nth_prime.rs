fn is_prime(num: u32) -> bool {
    for i in 2..=(num as f32).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate = 2;
    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(nth(5), 13);
        assert_eq!(nth(10000), 104_743);
    }
}