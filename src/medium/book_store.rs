pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = books.iter().fold([0; 5], |mut acc, b| {
        acc[(b - 1) as usize] += 1;
        acc
    });

    let mut bundle_sizes = Vec::new();
    while counts.iter().any(|&c| c > 0) {
        let mut current_bundle = 0;
        for count in counts.iter_mut() {
            if *count > 0 {
                *count -= 1;
                current_bundle += 1;
            }
        }
        bundle_sizes.push(current_bundle);
    }

    // (5,3) -> (4,4)
    for i in 0..bundle_sizes.len() {
        if bundle_sizes[i] == 5 {
            if let Some(j) = bundle_sizes[i+1..].iter().position(|&s| s == 3) {
                bundle_sizes[i] = 4;
                bundle_sizes[i + j + 1] = 4;
            }
        }
    }

    bundle_sizes.iter().map(|&size| match size {
        1 => 800,
        2 => (800f32 * 2f32 * 0.95).round() as u32,
        3 => (800f32 * 3f32 * 0.90).round() as u32,
        4 => (800f32 * 4f32 * 0.80).round() as u32,
        5 => (800f32 * 5f32 * 0.75).round() as u32,
        _ => 0,
    }).sum() 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_single_book() {
        let input = &[1];
        let output = lowest_price(input);
        let expected = 800;
        assert_eq!(output, expected);
    }

    #[test]
    fn two_of_the_same_book() {
        let input = &[2, 2];
        let output = lowest_price(input);
        let expected = 1600;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_basket() {
        let input = &[];
        let output = lowest_price(input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn two_different_books() {
        let input = &[1, 2];
        let output = lowest_price(input);
        let expected = 1520;
        assert_eq!(output, expected);
    }

    #[test]
    fn three_different_books() {
        let input = &[1, 2, 3];
        let output = lowest_price(input);
        let expected = 2160;
        assert_eq!(output, expected);
    }

    #[test]
    fn four_different_books() {
        let input = &[1, 2, 3, 4];
        let output = lowest_price(input);
        let expected = 2560;
        assert_eq!(output, expected);
    }

    #[test]
    fn five_different_books() {
        let input = &[1, 2, 3, 4, 5];
        let output = lowest_price(input);
        let expected = 3000;
        assert_eq!(output, expected);
    }

    #[test]
    fn two_groups_of_four_is_cheaper_than_group_of_five_plus_group_of_three() {
        let input = &[1, 1, 2, 2, 3, 3, 4, 5];
        let output = lowest_price(input);
        let expected = 5120;
        assert_eq!(output, expected);
    }

    #[test]
    fn two_groups_of_four_is_cheaper_than_groups_of_five_and_three() {
        let input = &[1, 1, 2, 3, 4, 4, 5, 5];
        let output = lowest_price(input);
        let expected = 5120;
        assert_eq!(output, expected);
    }

    #[test]
    fn group_of_four_plus_group_of_two_is_cheaper_than_two_groups_of_three() {
        let input = &[1, 1, 2, 2, 3, 4];
        let output = lowest_price(input);
        let expected = 4080;
        assert_eq!(output, expected);
    }

    #[test]
    fn two_each_of_first_four_books_and_one_copy_each_of_rest() {
        let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5];
        let output = lowest_price(input);
        let expected = 5560;
        assert_eq!(output, expected);
    }

    #[test]
    fn two_copies_of_each_book() {
        let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        let output = lowest_price(input);
        let expected = 6000;
        assert_eq!(output, expected);
    }

    #[test]
    fn three_copies_of_first_book_and_two_each_of_remaining() {
        let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1];
        let output = lowest_price(input);
        let expected = 6800;
        assert_eq!(output, expected);
    }

    #[test]
    fn three_each_of_first_two_books_and_two_each_of_remaining_books() {
        let input = &[1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 1, 2];
        let output = lowest_price(input);
        let expected = 7520;
        assert_eq!(output, expected);
    }

    #[test]
    fn four_groups_of_four_are_cheaper_than_two_groups_each_of_five_and_three() {
        let input = &[1, 1, 2, 2, 3, 3, 4, 5, 1, 1, 2, 2, 3, 3, 4, 5];
        let output = lowest_price(input);
        let expected = 10240;
        assert_eq!(output, expected);
    }

    #[test]
    fn check_that_groups_of_four_are_created_properly_even_when_there_are_more_groups_of_three_than_groups_of_five()
    {
        let input = &[1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 5, 5,];
        let output = lowest_price(input);
        let expected = 14560;
        assert_eq!(output, expected);
    }

    #[test]
    fn one_group_of_one_and_four_is_cheaper_than_one_group_of_two_and_three() {
        let input = &[1, 1, 2, 3, 4];
        let output = lowest_price(input);
        let expected = 3360;
        assert_eq!(output, expected);
    }

    #[test]
    fn one_group_of_one_and_two_plus_three_groups_of_four_is_cheaper_than_one_group_of_each_size() {
        let input = &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5];
        let output = lowest_price(input);
        let expected = 10000;
        assert_eq!(output, expected);
    }
}