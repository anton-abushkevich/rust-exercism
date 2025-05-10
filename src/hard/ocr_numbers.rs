#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = validate(input)?;
    let mut result = String::new();

    for chunk in lines.chunks(4) {
        if !result.is_empty() {
            result.push(',');
        }

        let digit_count = chunk[0].len() / 3;
        for i in 0..digit_count {
            let digit = std::array::from_fn(|j| &chunk[j][i*3..i*3+3]);
            result.push(match digit {
                [" _ ", "| |", "|_|", "   "] => '0',
                ["   ", "  |", "  |", "   "] => '1',
                [" _ ", " _|", "|_ ", "   "] => '2',
                [" _ ", " _|", " _|", "   "] => '3',
                ["   ", "|_|", "  |", "   "] => '4',
                [" _ ", "|_ ", " _|", "   "] => '5',
                [" _ ", "|_ ", "|_|", "   "] => '6',
                [" _ ", "  |", "  |", "   "] => '7',
                [" _ ", "|_|", "|_|", "   "] => '8',
                [" _ ", "|_|", " _|", "   "] => '9',
                _ => '?',
            });
        }
    }

    Ok(result)
}

fn validate(input: &str) -> Result<Vec<&str>, Error> {
    let lines: Vec<&str> = input.lines().collect();
    let row_count = lines.len();

    if row_count % 4 != 0 {
        return Err(Error::InvalidRowCount(row_count));
    }

    for line in &lines {
        if line.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
    }

    Ok(lines)
}

mod r {
    use super::*;

    #[test]
    fn input_with_lines_not_multiple_of_four_is_error() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           "| |\n" +
                           "   ";
        assert_eq!(Err(Error::InvalidRowCount(3)), convert(&input));
    }

    #[test]
    fn input_with_columns_not_multiple_of_three_is_error() {
        #[rustfmt::skip]
        let input = "    \n".to_string() +
                           "   |\n" +
                           "   |\n" +
                           "    ";
        assert_eq!(Err(Error::InvalidColumnCount(4)), convert(&input));
    }

    #[test]
    fn unrecognized_characters_return_question_mark() {
        #[rustfmt::skip]
        let input = "   \n".to_string() +
                           "  _\n" +
                           "  |\n" +
                           "   ";
        assert_eq!(Ok("?".to_string()), convert(&input));
    }

    #[test]
    fn recognizes_0() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           "| |\n" +
                           "|_|\n" +
                           "   ";
        assert_eq!(Ok("0".to_string()), convert(&input));
    }

    #[test]
    fn recognizes_1() {
        #[rustfmt::skip]
    let input = "   \n".to_string() +
                       "  |\n" +
                       "  |\n" +
                       "   ";
        assert_eq!(Ok("1".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_2() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           " _|\n" +
                           "|_ \n" +
                           "   ";
        assert_eq!(Ok("2".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_3() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           " _|\n" +
                           " _|\n" +
                           "   ";
        assert_eq!(Ok("3".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_4() {
        #[rustfmt::skip]
        let input = "   \n".to_string() +
                           "|_|\n" +
                           "  |\n" +
                           "   ";
        assert_eq!(Ok("4".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_5() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           "|_ \n" +
                           " _|\n" +
                           "   ";
        assert_eq!(Ok("5".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_6() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           "|_ \n" +
                           "|_|\n" +
                           "   ";
        assert_eq!(Ok("6".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_7() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           "  |\n" +
                           "  |\n" +
                           "   ";
        assert_eq!(Ok("7".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_8() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           "|_|\n" +
                           "|_|\n" +
                           "   ";
        assert_eq!(Ok("8".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_9() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                           "|_|\n" +
                           " _|\n" +
                           "   ";
        assert_eq!(Ok("9".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_110101100() {
        #[rustfmt::skip]
        let input = "       _     _        _  _ \n".to_string() +
                           "  |  || |  || |  |  || || |\n" +
                           "  |  ||_|  ||_|  |  ||_||_|\n" +
                           "                           ";
        assert_eq!(Ok("110101100".to_string()), convert(&input));
    }
    #[test]
    fn replaces_only_garbled_numbers_with_question_mark() {
        #[rustfmt::skip]
        let input = "       _     _           _ \n".to_string() +
                           "  |  || |  || |     || || |\n" +
                           "  |  | _|  ||_|  |  ||_||_|\n" +
                           "                           ";
        assert_eq!(Ok("11?10?1?0".to_string()), convert(&input));
    }
    #[test]
    fn recognizes_string_of_decimal_numbers() {
        #[rustfmt::skip]
        let input = "    _  _     _  _  _  _  _  _ \n".to_string() +
                           "  | _| _||_||_ |_   ||_||_|| |\n" +
                           "  ||_  _|  | _||_|  ||_| _||_|\n" +
                           "                              ";
        assert_eq!(Ok("1234567890".to_string()), convert(&input));
    }
    #[test]
    fn numbers_across_multiple_lines_are_joined_by_commas() {
        #[rustfmt::skip]
        let input = "    _  _ \n".to_string() +
                           "  | _| _|\n" +
                           "  ||_  _|\n" +
                           "         \n" +
                           "    _  _ \n" +
                           "|_||_ |_ \n" +
                           "  | _||_|\n" +
                           "         \n" +
                           " _  _  _ \n" +
                           "  ||_||_|\n" +
                           "  ||_| _|\n" +
                           "         ";
        assert_eq!(Ok("123,456,789".to_string()), convert(&input));
    }
}
