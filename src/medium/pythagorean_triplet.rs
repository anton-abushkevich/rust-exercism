use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res = HashSet::new();
    if sum % 2 == 1 { return res }
    
    for m in 2..=(sum / 2).isqrt() {
        for n in (m % 2 + 1..m).step_by(2) {
            if are_coprime(m, n) {
                let denominator = 2 * m * (m + n);
                if sum % denominator == 0 {
                    let k = sum / denominator;
                    let a = k * (m*m - n*n);
                    let b = 2 * k * m * n;
                    let c = k * (m*m + n*n);
                    let mut triplet = [a, b, c];
                    triplet.sort();
                    res.insert(triplet);
                } 
            }
        }
    }
    
    res
}

fn are_coprime(m: u32, n: u32) -> bool {
    let gcd = |mut a, mut b| {
        while b != 0 {
            (a, b) = (b, a % b)
        }
        a
    };
    gcd(m, n) == 1
}

// community solution
pub fn _find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    for a in 1..=sum / 3 {
        let den = 2 * (sum - a);
        let num = sum * sum - 2 * a * sum;
        if num % den == 0 {
            let b = num / den;
            if a < b {
                triplets.insert([a, b, sum - a - b]);
            }
        }
    }
    triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    #[test]
    fn triplets_whose_sum_is_12() {
        let input = 12;
        let output = find(input);
        let expected = [[3, 4, 5]];
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn triplets_whose_sum_is_108() {
        let input = 108;
        let output = find(input);
        let expected = [[27, 36, 45]];
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn triplets_whose_sum_is_1000() {
        let input = 1_000;
        let output = find(input);
        let expected = [[200, 375, 425]];
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn no_matching_triplets_for_1001() {
        let input = 1_001;
        let output = find(input);
        let expected = [];
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn returns_all_matching_triplets() {
        let input = 90;
        let output = find(input);
        let expected = [[9, 40, 41], [15, 36, 39]];
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn several_matching_triplets() {
        let input = 840;
        let output = find(input);
        let expected = [
            [40, 399, 401],
            [56, 390, 394],
            [105, 360, 375],
            [120, 350, 370],
            [140, 336, 364],
            [168, 315, 357],
            [210, 280, 350],
            [240, 252, 348],
        ];
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn triplets_for_large_number() {
        let input = 30_000;
        let output = find(input);
        let expected = [
            [1_200, 14_375, 14_425],
            [1_875, 14_000, 14_125],
            [5_000, 12_000, 13_000],
            [6_000, 11_250, 12_750],
            [7_500, 10_000, 12_500],
        ];
        let expected: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(output, expected);
    }

}