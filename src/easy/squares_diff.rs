pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
    //(1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
    //(1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n).abs_diff(sum_of_squares(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(difference(2), 4);
        assert_eq!(difference(10), 2640);
    }
}