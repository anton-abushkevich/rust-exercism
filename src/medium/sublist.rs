#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        _ => {
            if first_list == second_list {
                Comparison::Equal
            } else if second_list.windows(first_list.len()).any(|window| window == first_list) {
                Comparison::Sublist
            } else if first_list.windows(second_list.len()).any(|window| window == second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lists() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_list_within_non_empty_list() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn non_empty_list_contains_empty_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn list_equals_itself() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn different_lists() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[2, 3, 4];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
}
