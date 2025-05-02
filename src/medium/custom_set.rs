use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Copy + Eq + Hash>(HashSet<T>);

impl<T> FromIterator<T> for CustomSet<T>
where
    T: Copy + Eq + Hash
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: Copy + Eq + Hash> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self(input.iter().copied().collect())
    }

    pub fn contains(&self, element: &T) -> bool {
        self.0.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.0.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        self.0.intersection(&other.0).copied().collect()
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        self.0.difference(&other.0).copied().collect()
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        self.0.union(&other.0).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_with_no_elements_are_empty() {
        let set = CustomSet::<i32>::new(&[]);
        assert!(set.is_empty());
    }

    #[test]
    fn sets_with_elements_are_not_empty() {
        let set = CustomSet::<i32>::new(&[1]);
        assert!(!set.is_empty());
    }

    #[test]
    fn nothing_is_contained_in_an_empty_set() {
        let set = CustomSet::<i32>::new(&[]);
        assert!(!set.contains(&1));
    }

    #[test]
    fn when_the_element_is_in_the_set() {
        let set = CustomSet::<i32>::new(&[1, 2, 3]);
        assert!(set.contains(&1));
    }

    #[test]
    fn when_the_element_is_not_in_the_set() {
        let set = CustomSet::<i32>::new(&[1, 2, 3]);
        assert!(!set.contains(&4));
    }

    #[test]
    fn empty_set_is_a_subset_of_another_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[]);
        assert!(set_1.is_subset(&set_2));
    }

    #[test]
    fn empty_set_is_a_subset_of_non_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[1]);
        assert!(set_1.is_subset(&set_2));
    }

    #[test]
    fn non_empty_set_is_not_a_subset_of_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[1]);
        let set_2 = CustomSet::<i32>::new(&[]);
        assert!(!set_1.is_subset(&set_2));
    }

    #[test]
    fn set_is_a_subset_of_set_with_exact_same_elements() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
        let set_2 = CustomSet::<i32>::new(&[1, 2, 3]);
        assert!(set_1.is_subset(&set_2));
    }

    #[test]
    fn set_is_a_subset_of_larger_set_with_same_elements() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
        let set_2 = CustomSet::<i32>::new(&[4, 1, 2, 3]);
        assert!(set_1.is_subset(&set_2));
    }

    #[test]
    fn set_is_not_a_subset_of_set_that_does_not_contain_its_elements() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
        let set_2 = CustomSet::<i32>::new(&[4, 1, 3]);
        assert!(!set_1.is_subset(&set_2));
    }

    #[test]
    fn the_empty_set_is_disjoint_with_itself() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[]);
        assert!(set_1.is_disjoint(&set_2));
    }

    #[test]
    fn empty_set_is_disjoint_with_non_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[1]);
        assert!(set_1.is_disjoint(&set_2));
    }

    #[test]
    fn non_empty_set_is_disjoint_with_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[1]);
        let set_2 = CustomSet::<i32>::new(&[]);
        assert!(set_1.is_disjoint(&set_2));
    }

    #[test]
    fn sets_are_not_disjoint_if_they_share_an_element() {
        let set_1 = CustomSet::<i32>::new(&[1, 2]);
        let set_2 = CustomSet::<i32>::new(&[2, 3]);
        assert!(!set_1.is_disjoint(&set_2));
    }

    #[test]
    fn sets_are_disjoint_if_they_share_no_elements() {
        let set_1 = CustomSet::<i32>::new(&[1, 2]);
        let set_2 = CustomSet::<i32>::new(&[3, 4]);
        assert!(set_1.is_disjoint(&set_2));
    }

    #[test]
    fn empty_sets_are_equal() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1, set_2);
    }

    #[test]
    fn empty_set_is_not_equal_to_non_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[1, 2, 3]);
        assert_ne!(set_1, set_2);
    }

    #[test]
    fn non_empty_set_is_not_equal_to_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
        let set_2 = CustomSet::<i32>::new(&[]);
        assert_ne!(set_1, set_2);
    }

    #[test]
    fn sets_with_the_same_elements_are_equal() {
        let set_1 = CustomSet::<i32>::new(&[1, 2]);
        let set_2 = CustomSet::<i32>::new(&[2, 1]);
        assert_eq!(set_1, set_2);
    }

    #[test]
    fn sets_with_different_elements_are_not_equal() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
        let set_2 = CustomSet::<i32>::new(&[1, 2, 4]);
        assert_ne!(set_1, set_2);
    }

    #[test]
    fn set_is_not_equal_to_larger_set_with_same_elements() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
        let set_2 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
        assert_ne!(set_1, set_2);
    }

    #[test]
    fn set_is_equal_to_a_set_constructed_from_an_array_with_duplicates() {
        let set_1 = CustomSet::<i32>::new(&[1]);
        let set_2 = CustomSet::<i32>::new(&[1, 1]);
        assert_eq!(set_1, set_2);
    }

    #[test]
    fn add_to_empty_set() {
        let mut set = CustomSet::<i32>::new(&[]);
        set.add(3);
        let expected = CustomSet::<i32>::new(&[3]);
        assert_eq!(set, expected);
    }

    #[test]
    fn add_to_non_empty_set() {
        let mut set = CustomSet::<i32>::new(&[1, 2, 4]);
        set.add(3);
        let expected = CustomSet::<i32>::new(&[1, 2, 3, 4]);
        assert_eq!(set, expected);
    }

    #[test]
    fn adding_an_existing_element_does_not_change_the_set() {
        let mut set = CustomSet::<i32>::new(&[1, 2, 3]);
        set.add(3);
        let expected = CustomSet::<i32>::new(&[1, 2, 3]);
        assert_eq!(set, expected);
    }

    #[test]
    fn intersection_of_two_empty_sets_is_an_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.intersection(&set_2), expected);
    }

    #[test]
    fn intersection_of_an_empty_set_and_non_empty_set_is_an_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[3, 2, 5]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.intersection(&set_2), expected);
    }

    #[test]
    fn intersection_of_a_non_empty_set_and_an_empty_set_is_an_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
        let set_2 = CustomSet::<i32>::new(&[]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.intersection(&set_2), expected);
    }

    #[test]
    fn intersection_of_two_sets_with_no_shared_elements_is_an_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
        let set_2 = CustomSet::<i32>::new(&[4, 5, 6]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.intersection(&set_2), expected);
    }

    #[test]
    fn intersection_of_two_sets_with_shared_elements_is_a_set_of_the_shared_elements() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
        let set_2 = CustomSet::<i32>::new(&[3, 2, 5]);
        let expected = CustomSet::<i32>::new(&[2, 3]);
        assert_eq!(set_1.intersection(&set_2), expected);
    }

    #[test]
    fn difference_of_two_empty_sets_is_an_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.difference(&set_2), expected);
    }

    #[test]
    fn difference_of_empty_set_and_non_empty_set_is_an_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[3, 2, 5]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.difference(&set_2), expected);
    }

    #[test]
    fn difference_of_a_non_empty_set_and_an_empty_set_is_the_non_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
        let set_2 = CustomSet::<i32>::new(&[]);
        let expected = CustomSet::<i32>::new(&[1, 2, 3, 4]);
        assert_eq!(set_1.difference(&set_2), expected);
    }

    #[test]
    fn difference_of_two_non_empty_sets_is_a_set_of_elements_that_are_only_in_the_first_set() {
        let set_1 = CustomSet::<i32>::new(&[3, 2, 1]);
        let set_2 = CustomSet::<i32>::new(&[2, 4]);
        let expected = CustomSet::<i32>::new(&[1, 3]);
        assert_eq!(set_1.difference(&set_2), expected);
    }

    #[test]
    fn difference_removes_all_duplicates_in_the_first_set() {
        let set_1 = CustomSet::<i32>::new(&[1, 1]);
        let set_2 = CustomSet::<i32>::new(&[1]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.difference(&set_2), expected);
    }

    #[test]
    fn union_of_empty_sets_is_an_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[]);
        let expected = CustomSet::<i32>::new(&[]);
        assert_eq!(set_1.union(&set_2), expected);
    }

    #[test]
    fn union_of_an_empty_set_and_non_empty_set_is_the_non_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[]);
        let set_2 = CustomSet::<i32>::new(&[2]);
        let expected = CustomSet::<i32>::new(&[2]);
        assert_eq!(set_1.union(&set_2), expected);
    }

    #[test]
    fn union_of_a_non_empty_set_and_empty_set_is_the_non_empty_set() {
        let set_1 = CustomSet::<i32>::new(&[1, 3]);
        let set_2 = CustomSet::<i32>::new(&[]);
        let expected = CustomSet::<i32>::new(&[1, 3]);
        assert_eq!(set_1.union(&set_2), expected);
    }

    #[test]
    fn union_of_non_empty_sets_contains_all_unique_elements() {
        let set_1 = CustomSet::<i32>::new(&[1, 3]);
        let set_2 = CustomSet::<i32>::new(&[2, 3]);
        let expected = CustomSet::<i32>::new(&[3, 2, 1]);
        assert_eq!(set_1.union(&set_2), expected);
    }
}