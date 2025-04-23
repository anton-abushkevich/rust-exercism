pub fn square(s: u32) -> u64 {
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_square() {
        assert_eq!(square(1), 1);
        assert_eq!(square(2), 2);
        assert_eq!(square(3), 4);
        assert_eq!(square(4), 8);
        assert_eq!(square(8), 128);
        assert_eq!(square(10), 512);
        assert_eq!(square(16), 32_768);
        assert_eq!(square(33), 4_294_967_296);
        assert_eq!(square(64), 9_223_372_036_854_775_808);
    }

    #[test]
    fn check_total() {
        assert_eq!(total(), 18_446_744_073_709_551_615)
    }
}