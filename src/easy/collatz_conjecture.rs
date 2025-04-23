pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None
    }
    let mut n = n;
    let mut count = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3*n + 1;
        }

        count += 1;
    }
    Some(count)
}

#[test]
fn test() {
    assert_eq!(collatz(1), Some(0));
    assert_eq!(collatz(16), Some(4));
    assert_eq!(collatz(12), Some(9));
    assert_eq!(collatz(1_000_000), Some(152));
    assert_eq!(collatz(0), None);
}