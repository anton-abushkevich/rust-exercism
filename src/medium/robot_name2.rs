// the problem with random names is that the more robots
// one have the harder it is to hit unique name with generator
// here is the solution that can handle all 26*26*1000 active Robots

use std::cell::RefCell;

thread_local! {
    static ROBOTS: RefCell<Vec<String>> = RefCell::new(
        (b'A'..=b'Z').flat_map(|ch1| {
            (b'A'..=b'Z').flat_map(move |ch2| {
                (0..1000).map(move |num| {
                    format!("{}{}{:03}", ch1 as char, ch2 as char, num)
                })
            })
        }).collect()
    );
}

#[derive(Debug)]
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Self(next_available())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = next_available();
    }
}

fn next_available() -> String {
    ROBOTS
        .with(|names| names.borrow_mut().pop())
        .expect("no more name combinations available")
}

//if the Robot is destroyed -> make its name available for the new one
impl Drop for Robot {
    fn drop(&mut self) {
        ROBOTS.with(|names| names.borrow_mut().push(self.0.clone()))
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
        let vec: Vec<_> = (0..3529).map(|_|  Robot::new()).collect();
        let set: HashSet<_> = vec.iter().map(|robot| robot.name()).collect();
        let number_of_collisions = vec.len() - set.len();
        assert_eq!(number_of_collisions, 0);
    }
}