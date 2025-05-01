use std::sync::atomic::{AtomicU16, Ordering};

static COUNTER: AtomicU16 = AtomicU16::new(0);

fn gen_name() -> String {
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let letter1 = (b'A' + (count / 26000) as u8) as char;
    let letter2 = (b'A' + ((count % 26000) / 1000) as u8) as char;
    let number = (count % 1000) as u32;
    format!("{}{}{:03}", letter1, letter2, number)
}

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Self(gen_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = gen_name()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_name_matches_pattern(n: &str) {
        assert!(n.len() == 5, "name is exactly 5 characters long");
        assert!(
            n[0..2].chars().all(|c| c.is_ascii_uppercase()),
            "name starts with 2 uppercase letters"
        );
        assert!(
            n[2..].chars().all(|c| c.is_ascii_digit()),
            "name ends with 3 numbers"
        );
    }

    #[test]
    fn name_should_match_expected_pattern() {
        let r =  Robot::new();
        assert_name_matches_pattern(r.name());
    }

    #[test]
    fn different_robots_have_different_names() {
        let r1 =  Robot::new();
        let r2 =  Robot::new();
        assert_ne!(r1.name(), r2.name(), "Robot names should be different");
    }

    #[test]
    fn new_name_should_match_expected_pattern() {
        let mut r =  Robot::new();
        assert_name_matches_pattern(r.name());
        r.reset_name();
        assert_name_matches_pattern(r.name());
    }

    #[test]
    fn new_name_is_different_from_old_name() {
        let mut r =  Robot::new();
        let n1 = r.name().to_string();
        r.reset_name();
        let n2 = r.name().to_string();
        assert_ne!(n1, n2, "Robot name should change when reset");
    }

    #[test]
    fn many_different_robots_have_different_names() {
        use std::collections::HashSet;
        // In 3,529 random robot names, there is ~99.99% chance of a name collision
        let vec: Vec<_> = (0..3529).map(|_|  crate::medium::robot_name2::Robot::new()).collect();
        let set: HashSet<_> = vec.iter().map(|robot| robot.name()).collect();
        let number_of_collisions = vec.len() - set.len();
        assert_eq!(number_of_collisions, 0);
    }
}