/// Fast inverse sqrt approximation algorithm (original: from Quake III Arena C-code)
pub fn q_rsqrt(number: f32) -> f32 {
    let half = number * 0.5f32;

    // f32 -> u32
    let i: u32 = unsafe { std::mem::transmute(number) };

    // wtf-formula
    let j = 0x5f3759df - (i >> 1);

    // u32 -> f32
    let y: f32 = unsafe { std::mem::transmute(j) };

    // one iteration of the Newton's sqrt approximation method
    y * (1.5f32 - (half * y * y))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // maximum permissible error (0.176%). worst case is ~0.175%, see test_worst_case_error()
    const EPSILON: f32 = 0.00176;
 
    fn assert_inv_sqrt_approx(x: f32) {
        let exact = 1.0 / x.sqrt();
        let approx = q_rsqrt(x);
        assert!(
            (approx - exact).abs() <= EPSILON * exact,
            "x = {}: approx = {}, exact = {}",
            x,
            approx,
            exact
        );
    }

    #[test]
    fn test_positive_numbers() {
        let test_cases = [0.01, 0.1, 1.0, 2.0, 10.0, 100.0, 1000.0];
        for &x in &test_cases {
            assert_inv_sqrt_approx(x);
        }
    }

    #[test]
    fn test_small_numbers() {
        let test_cases = [1e-6, 1e-5, 1e-4];
        for &x in &test_cases {
            assert_inv_sqrt_approx(x);
        }
    }

    #[test]
    fn test_large_numbers() {
        let test_cases = [1e6, 1e7, 1e8];
        for &x in &test_cases {
            assert_inv_sqrt_approx(x);
        }
    }

    #[test]
    fn test_edge_cases() {
        assert_inv_sqrt_approx(f32::MIN_POSITIVE);
        assert_inv_sqrt_approx(f32::MAX);
    }

    #[test]
    fn test_random_values() {
        use rand::Rng;
        let mut rng = rand::rng();
        for _ in 0..100 {
            let x = rng.random_range(0.001..1000.0);
            assert_inv_sqrt_approx(x);
        }
    }

    #[test]
    fn test_worst_case_error() {
        let mut max_error = 0.0f32;
        let mut worst_x = 0.0f32;

        // find the worst error from 0.001 to 1_000
        for x in (1..=1_000).map(|i| i as f32 * 0.001) {
            let exact = 1.0 / x.sqrt();
            let approx = q_rsqrt(x);
            let error = (approx - exact).abs() / exact;

            if error > max_error {
                max_error = error;
                worst_x = x;
            }
        }

        // worst approximation should be ~0.175%
        //println!("Maximum error: {:.3}% for x = {}", max_error * 100.0, worst_x);
        assert!(max_error <= EPSILON, 
                "The permissible error has been exceeded for x = {} ({})", worst_x, max_error);
    }    
}