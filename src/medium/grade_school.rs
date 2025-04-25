use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.0.values().any(|students| students.contains(student)) {
            self.0.entry(grade).or_default().insert(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0.get(&grade).map_or_else(Vec::new, |students| {
            students.iter().cloned().collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_is_empty_if_no_students_in_the_roster() {
        let s = School::new();
        assert_eq!(s.grade(1), Vec::<String>::new())
    }

    #[test]
    fn grade_is_empty_if_no_students_in_that_grade() {
        let mut s = School::new();
        s.add(2, "Peter");
        s.add(2, "Zoe");
        s.add(2, "Alex");
        s.add(3, "Jim");
        assert_eq!(s.grade(1), Vec::<String>::new())
    }

    #[test]
    fn student_not_added_to_same_grade_more_than_once() {
        let mut s = School::new();
        s.add(2, "Blair");
        s.add(2, "James");
        s.add(2, "James");
        s.add(2, "Paul");
        assert_eq!(s.grade(2), vec!["Blair", "James", "Paul"])
    }

    #[test]
    fn student_not_added_to_multiple_grades() {
        let mut s = School::new();
        s.add(2, "Blair");
        s.add(2, "James");
        s.add(3, "James");
        s.add(3, "Paul");
        assert_eq!(s.grade(2), vec!["Blair", "James"])
    }

    #[test]
    fn student_not_added_to_other_grade_for_multiple_grades() {
        let mut s = School::new();
        s.add(2, "Blair");
        s.add(2, "James");
        s.add(3, "James");
        s.add(3, "Paul");
        assert_eq!(s.grade(3), vec!["Paul"])
    }

    #[test]
    fn students_are_sorted_by_name_in_a_grade() {
        let mut s = School::new();
        s.add(5, "Franklin");
        s.add(5, "Bradley");
        s.add(1, "Jeff");
        assert_eq!(s.grade(5), vec!["Bradley", "Franklin"])
    }

    #[test]
    fn grades_for_empty_school() {
        let s = School::new();
        assert_eq!(s.grades(), Vec::<u32>::new());
    }

    #[test]
    fn grades_for_one_student() {
        let mut s = School::new();
        s.add(2, "Aimee");
        assert_eq!(s.grades(), vec![2])
    }

    #[test]
    fn grades_for_several_students_are_sorted() {
        let mut s = School::new();
        s.add(2, "Aimee");
        s.add(7, "Logan");
        s.add(4, "Blair");
        assert_eq!(s.grades(), vec![2, 4, 7])
    }

    #[test]
    fn grades_when_several_students_have_the_same_grade() {
        let mut s = School::new();
        s.add(2, "Aimee");
        s.add(2, "Logan");
        s.add(2, "Blair");
        assert_eq!(s.grades(), vec![2])
    }
}