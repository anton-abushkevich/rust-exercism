use std::cmp::{Ordering, max};
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone)]
pub struct Decimal {
    sign: i8,
    digits: Vec<u8>,
    point: usize,
}

impl Decimal {
    pub fn new(sign: i8, digits: Vec<u8>, point: usize) -> Self {
        let mut d = Decimal { sign, digits, point };
        d.normalize();
        d
    }

    pub fn try_from(s: &str) -> Option<Self> {
        let mut sign = 1;
        let mut digits = Vec::new();
        let mut point = None;

        let mut chars = s.chars().peekable();

        match chars.peek() {
            Some('+') => { chars.next(); }
            Some('-') => {
                sign = -1;
                chars.next();
            }
            _ => {}
        }

        let mut digit_count = 0;
        for c in chars {
            if c == '.' {
                if point.is_some() { return None; } 
                point = Some(digit_count);
                continue;
            }

            let d = match c.to_digit(10) {
                Some(d) => d as u8,
                None => return None,
            };

            digits.push(d);
            digit_count += 1;
        }

        let point = point.unwrap_or(digit_count);
        Some(Decimal::new(sign, digits, point))
    }

    fn normalize(&mut self) {
        let leading_zeros = self.digits.iter()
            .take(self.point.saturating_sub(1))
            .take_while(|&&d| d == 0)
            .count();

        if leading_zeros > 0 {
            self.digits.drain(0..leading_zeros);
            self.point -= leading_zeros;
        }

        while let Some(&0) = self.digits[self.point..].last() {
            self.digits.pop();
        }

        if self.digits.is_empty() {
            self.sign = 0;
            self.point = 0;
            self.digits.push(0);
        }
    }

    fn normalize_carry(&mut self) {
        let mut carry = 0;
        for digit in self.digits.iter_mut().rev() {
            let new_val = *digit + carry;
            *digit = new_val % 10;
            carry = new_val / 10;
        }

        if carry > 0 {
            self.digits.insert(0, carry);
            self.point += 1;
        }
        self.normalize();
    }
    
    fn align_with(&self, other: &Self) -> (Vec<u8>, Vec<u8>, usize) {
        let left = max(self.point, other.point);
        let right = max(self.digits.len() - self.point, other.digits.len() - other.point);

        let mut a = vec![0; left - self.point];
        a.extend(&self.digits);
        a.extend(vec![0; right - (self.digits.len() - self.point)]);

        let mut b = vec![0; left - other.point];
        b.extend(&other.digits);
        b.extend(vec![0; right - (other.digits.len() - other.point)]);

        (a, b, left)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        if self.sign != other.sign { return false; }
        self.digits == other.digits && self.point == other.point
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.sign, other.sign) {
            (0, 0) => Some(Ordering::Equal),
            (_, 0) => if self.sign > 0 { Some(Ordering::Greater) } else { Some(Ordering::Less) },
            (0, _) => if other.sign > 0 { Some(Ordering::Less) } else { Some(Ordering::Greater) },
            (a, b) if a != b => a.partial_cmp(&b),
            _ => {
                let (a, b, _) = self.align_with(other);
                let cmp = a.cmp(&b);
                if self.sign > 0 { Some(cmp) } else { Some(cmp.reverse()) }
            }
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.sign, other.sign) {
            (0, _) => other,
            (_, 0) => self,
            (a, b) if a == b => {
                let (a_digits, b_digits, point) = self.align_with(&other);
                let sum = add_digits(&a_digits, &b_digits);
                let mut result = Decimal::new(self.sign, sum, point);
                result.normalize_carry();
                result
            }
            _ => self - Decimal::new(-other.sign, other.digits, other.point),
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self.sign, other.sign) {
            (0, _) => Decimal::new(-other.sign, other.digits, other.point),
            (_, 0) => self,
            (a, b) if a != b => {
                let (a_digits, b_digits, point) = self.align_with(&other);
                let sum = add_digits(&a_digits, &b_digits);
                Decimal::new(self.sign, sum, point)
            }
            _ => {
                let (a_digits, b_digits, point) = self.align_with(&other);
                if a_digits >= b_digits {
                    let diff = sub_digits(&a_digits, &b_digits);
                    Decimal::new(self.sign, diff, point)
                } else {
                    let diff = sub_digits(&b_digits, &a_digits);
                    Decimal::new(-self.sign, diff, point)
                }
            }
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.digits == [0] || other.digits == [0] {
            return Decimal::new(0, vec![0], 0);
        }

        let product = mul_digits(&self.digits, &other.digits);
        let point = self.point + other.point;
        let sign = self.sign * other.sign;

        Decimal::new(sign, product, point)
    }
}

fn add_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(a.len().max(b.len()) + 1);
    let mut carry = 0;

    for (a_digit, b_digit) in a.iter().rev().zip(b.iter().rev()) {
        let sum = a_digit + b_digit + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }

    if carry > 0 {
        result.push(carry);
    }

    result.reverse();
    result
}

fn sub_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut borrow = 0;

    for (a, b) in a.iter().rev().zip(b.iter().rev()) {
        let mut diff = (*a as i8) - (*b as i8) - borrow;
        borrow = 0;

        if diff < 0 {
            diff += 10;
            borrow = 1;
        }

        result.push(diff as u8);
    }

    result.reverse();
    result
}

fn mul_digits(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![0; a.len() + b.len()];

    for (i, a_digit) in a.iter().rev().enumerate() {
        let mut carry = 0;

        for (j, b_digit) in b.iter().rev().enumerate() {
            let product = a_digit * b_digit + carry + result[i + j];
            result[i + j] = product % 10;
            carry = product / 10;
        }

        if carry > 0 {
            result[i + b.len()] += carry;
        }
    }

    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a Decimal from a string literal
    ///
    /// Use only when you _know_ that your value is valid.
    fn decimal(input: &str) -> Decimal {
        Decimal::try_from(input).expect("That was supposed to be a valid value")
    }

    /// Some big and precise values we can use for testing. [0] + [1] == [2]
    const BIGS: [&str; 3] = [
        "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
        "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
        "200000000000000000000000000000000000000000000.00000000000000000000000000000000000000003",
    ];
    
    // test simple properties of required operations
    #[test]
    fn eq() {
        assert!(decimal("0.0") == decimal("0.0"));
        assert!(decimal("1.0") == decimal("1.0"));
        for big in BIGS.iter() {
            assert!(decimal(big) == decimal(big));
        }
    }

    #[test]
    fn ne() {
        assert!(decimal("0.0") != decimal("1.0"));
        assert!(decimal(BIGS[0]) != decimal(BIGS[1]));
    }

    #[test]
    fn gt() {
        for slice_2 in BIGS.windows(2) {
            assert!(decimal(slice_2[1]) > decimal(slice_2[0]));
        }

    }

    #[test]
    fn lt() {
        for slice_2 in BIGS.windows(2) {
            assert!(decimal(slice_2[0]) < decimal(slice_2[1]));
        }

    }

    #[test]
    fn add() {
        assert_eq!(decimal("0.1") + decimal("0.2"), decimal("0.3"));
        assert_eq!(decimal(BIGS[0]) + decimal(BIGS[1]), decimal(BIGS[2]));
        assert_eq!(decimal(BIGS[1]) + decimal(BIGS[0]), decimal(BIGS[2]));
    }

    #[test]
    fn sub() {
        assert_eq!(decimal(BIGS[2]) - decimal(BIGS[1]), decimal(BIGS[0]));
        assert_eq!(decimal(BIGS[2]) - decimal(BIGS[0]), decimal(BIGS[1]));
    }

    #[test]
    fn mul() {
        for big in BIGS.iter() {
            assert_eq!(decimal(big) * decimal("2"), decimal(big) + decimal(big));
        }

    }

    // test identities
    #[test]
    fn add_id() {
        assert_eq!(decimal("1.0") + decimal("0.0"), decimal("1.0"));
        assert_eq!(decimal("0.1") + decimal("0.0"), decimal("0.1"));
        assert_eq!(decimal("0.0") + decimal("1.0"), decimal("1.0"));
        assert_eq!(decimal("0.0") + decimal("0.1"), decimal("0.1"));
    }

    #[test]
    fn sub_id() {
        assert_eq!(decimal("1.0") - decimal("0.0"), decimal("1.0"));
        assert_eq!(decimal("0.1") - decimal("0.0"), decimal("0.1"));
    }

    #[test]
    fn mul_id() {
        assert_eq!(decimal("2.1") * decimal("1.0"), decimal("2.1"));
        assert_eq!(decimal("1.0") * decimal("2.1"), decimal("2.1"));
    }

    #[test]
    fn gt_positive_and_zero() {
        assert!(decimal("1.0") > decimal("0.0"));
        assert!(decimal("0.1") > decimal("0.0"));
    }

    #[test]
    fn gt_negative_and_zero() {
        assert!(decimal("0.0") > decimal("-0.1"));
        assert!(decimal("0.0") > decimal("-1.0"));
    }

    #[test]
    fn unequal_number_of_decimal_places() {
        assert!(decimal("3.14") > decimal("3.13"));
        assert!(decimal("3.14") > decimal("3.131"));
        assert!(decimal("3.14") > decimal("3.1"));
        assert!(decimal("3.13") < decimal("3.14"));
        assert!(decimal("3.131") < decimal("3.14"));
        assert!(decimal("3.1") < decimal("3.14"));
    }

    // tests of arbitrary precision behavior
    #[test]
    fn add_uneven_position() {
        assert_eq!(decimal("0.1") + decimal("0.02"), decimal("0.12"));
    }

    #[test]
    fn eq_vary_sig_digits() {
        assert!(decimal("0") == decimal("0000000000000.0000000000000000000000"));
        assert!(decimal("1") == decimal("00000000000000001.000000000000000000"));
    }

    #[test]
    fn add_vary_precision() {
        assert_eq!(
            decimal("100000000000000000000000000000000000000000000")
                + decimal("0.00000000000000000000000000000000000000001"),
            decimal(BIGS[0])
        )
    }

    #[test]
    fn cleanup_precision() {
        assert_eq!(
            decimal("10000000000000000000000000000000000000000000000.999999999999999999999999998",)
                + decimal(
                "10000000000000000000000000000000000000000000000.000000000000000000000000002",
            ),
            decimal("20000000000000000000000000000000000000000000001")
        )
    }

    #[test]
    fn gt_varying_positive_precisions() {
        assert!(decimal("1.1") > decimal("1.01"));
        assert!(decimal("1.01") > decimal("1.0"));
        assert!(decimal("1.0") > decimal("0.1"));
        assert!(decimal("0.1") > decimal("0.01"));
    }

    #[test]
    fn gt_positive_and_negative() {
        assert!(decimal("1.0") > decimal("-1.0"));
        assert!(decimal("1.1") > decimal("-1.1"));
        assert!(decimal("0.1") > decimal("-0.1"));
    }

    #[test]
    fn gt_varying_negative_precisions() {
        assert!(decimal("-0.01") > decimal("-0.1"));
        assert!(decimal("-0.1") > decimal("-1.0"));
        assert!(decimal("-1.0") > decimal("-1.01"));
        assert!(decimal("-1.01") > decimal("-1.1"));
    }

    // test signed properties
    #[test]
    fn negatives() {
        assert!(Decimal::try_from("-1").is_some());
        assert_eq!(decimal("0") - decimal("1"), decimal("-1"));
        assert_eq!(decimal("5.5") + decimal("-6.5"), decimal("-1"));
    }

    #[test]
    fn explicit_positive() {
        assert_eq!(decimal("+1"), decimal("1"));
        assert_eq!(decimal("+2.0") - decimal("-0002.0"), decimal("4"));
    }

    #[test]
    fn multiply_by_negative() {
        assert_eq!(decimal("5") * decimal("-0.2"), decimal("-1"));
        //assert_eq!(decimal("-20") * decimal("-0.2"), decimal("4"));
    }

    #[test]
    fn simple_partial_cmp() {
        assert!(decimal("1.0") < decimal("1.1"));
        assert!(decimal("0.00000000000000000000001") > decimal("-20000000000000000000000000000"));
    }

    // test carrying rules
    // these tests are designed to ensure correctness of implementations for which the
    // integer and fractional parts of the number are stored separately
    #[test]
    fn carry_into_integer() {
        assert_eq!(decimal("0.901") + decimal("0.1"), decimal("1.001"))
    }

    #[test]
    fn carry_into_fractional_with_digits_to_right() {
        assert_eq!(decimal("0.0901") + decimal("0.01"), decimal("0.1001"))
    }

    #[test]
    fn add_carry_over_negative() {
        assert_eq!(decimal("-1.99") + decimal("-0.01"), decimal("-2.0"))
    }

    #[test]
    fn sub_carry_over_negative() {
        assert_eq!(decimal("-1.99") - decimal("0.01"), decimal("-2.0"))
    }

    #[test]
    fn add_carry_over_negative_with_fractional() {
        assert_eq!(decimal("-1.99") + decimal("-0.02"), decimal("-2.01"))
    }

    #[test]
    fn sub_carry_over_negative_with_fractional() {
        assert_eq!(decimal("-1.99") - decimal("0.02"), decimal("-2.01"))
    }

    #[test]
    fn carry_from_rightmost_one() {
        assert_eq!(decimal("0.09") + decimal("0.01"), decimal("0.1"))
    }

    #[test]
    fn carry_from_rightmost_more() {
        assert_eq!(decimal("0.099") + decimal("0.001"), decimal("0.1"))
    }

    #[test]
    fn carry_from_rightmost_into_integer() {
        assert_eq!(decimal("0.999") + decimal("0.001"), decimal("1.0"))
    }

    // test arithmetic borrow rules
    #[test]
    fn add_borrow() {
        assert_eq!(decimal("0.01") + decimal("-0.0001"), decimal("0.0099"))
    }

    #[test]
    fn sub_borrow() {
        assert_eq!(decimal("0.01") - decimal("0.0001"), decimal("0.0099"))
    }

    #[test]
    fn add_borrow_integral() {
        assert_eq!(decimal("1.0") + decimal("-0.01"), decimal("0.99"))
    }

    #[test]
    fn sub_borrow_integral() {
        assert_eq!(decimal("1.0") - decimal("0.01"), decimal("0.99"))
    }

    #[test]
    fn add_borrow_integral_zeroes() {
        assert_eq!(decimal("1.0") + decimal("-0.99"), decimal("0.01"))
    }

    #[test]
    fn sub_borrow_integral_zeroes() {
        assert_eq!(decimal("1.0") - decimal("0.99"), decimal("0.01"))
    }

    #[test]
    fn borrow_from_negative() {
        assert_eq!(decimal("-1.0") + decimal("0.01"), decimal("-0.99"))
    }

    #[test]
    fn add_into_fewer_digits() {
        assert_eq!(decimal("0.011") + decimal("-0.001"), decimal("0.01"))
    }

    // misc tests of arithmetic properties
    #[test]
    fn sub_into_fewer_digits() {
        assert_eq!(decimal("0.011") - decimal("0.001"), decimal("0.01"))
    }

    #[test]
    fn add_away_decimal() {
        assert_eq!(decimal("1.1") + decimal("-0.1"), decimal("1.0"))
    }

    #[test]
    fn sub_away_decimal() {
        assert_eq!(decimal("1.1") - decimal("0.1"), decimal("1.0"))
    }
}