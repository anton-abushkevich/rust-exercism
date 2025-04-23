#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64 / 31_557_600.0)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::period()
    }

    fn period() -> f64;
}

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n;
        impl Planet for $n {
            fn period() -> f64 { $p }
        }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();
        let delta: f64 = 0.01;
        if diff > delta {
            panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
        }
    }

    #[test]
    fn age_on_earth() {
        let seconds = 1_000_000_000;
        let duration = Duration::from(seconds);
        let output = Earth::years_during(&duration);
        let expected = 31.69;
        assert_in_delta(expected, output);
    }

    #[test]
    fn age_on_mercury() {
        let seconds = 2_134_835_688;
        let duration = Duration::from(seconds);
        let output = Mercury::years_during(&duration);
        let expected = 280.88;
        assert_in_delta(expected, output);
    }
}
