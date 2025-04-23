pub fn _sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let calc_multiples = |factor: &u32| {
        let mut i = *factor;
        let mut multiples: Vec<u32> = vec![];
        while i < limit {
            multiples.push(i);
            i += factor;
        }
        multiples
    };
    let mut combined_multiples: Vec<u32> = vec![];
    for (_, f) in factors.iter().enumerate() {
        let multiples = calc_multiples(f);
        let set: std::collections::HashSet<_> = combined_multiples.into_iter().chain(multiples).collect();
        combined_multiples = set.into_iter().collect();
    }
    combined_multiples.iter().sum()
}

pub fn sum_of_multiples(limit: u32, divs: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| divs.iter().any(|d| x % d == 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(sum_of_multiples(10, &[3, 5]), 23);
        assert_eq!(sum_of_multiples(100, &[3, 5]), 2_318);
        assert_eq!(sum_of_multiples(10000, &[2, 3, 5, 7, 11]), 39_614_537);
    }
}
