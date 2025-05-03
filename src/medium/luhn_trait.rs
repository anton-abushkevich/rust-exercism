pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .try_fold((0, 0), |(i, checksum), c| {
                c.to_digit(10)
                    .map(|d| if i % 2 == 0 { d } else { d * 2 })
                    .map(|d| checksum + (if d > 9 { d - 9 } else { d }))
                    .map(|d| (i + 1, d))
            })
            .is_some_and(|(len, checksum)| len > 1 && checksum % 10 == 0)        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_validate_from_a_str() {
        assert!("046 454 286".valid_luhn());
        assert!(!"046 454 287".valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_string() {
        assert!(String::from("046 454 286").valid_luhn());
        assert!(!String::from("046 454 287").valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u8() {
        assert!(240u8.valid_luhn());
        assert!(!241u8.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u16() {
        let valid = 64_436u16;
        let invalid = 64_437u16;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u32() {
        let valid = 46_454_286u32;
        let invalid = 46_454_287u32;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u64() {
        let valid = 8273_1232_7352_0562u64;
        let invalid = 8273_1232_7352_0569u64;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_usize() {
        let valid = 8273_1232_7352_0562usize;
        let invalid = 8273_1232_7352_0569usize;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn input_digit_9_is_still_correctly_converted_to_output_digit_9() {
        assert!("091".valid_luhn());
    }
}