pub fn count(lines: &[&str]) -> u32 {
    let mut vertices = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.char_indices() {
            if c == '+' {
                vertices.push((i, j))
            }
        }
    }

    if vertices.len() < 4 { return 0; }

    let sides_check =
        |x: usize, y: usize, c: u8| lines[x].as_bytes()[y] == b'+' || lines[x].as_bytes()[y] == c;

    let mut count = 0;
    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        for vertex in vertices.iter().skip(i + 1) {
            let (x2, y2) = *vertex;

            if x1 == x2 || y1 == y2 { continue; }

            let top_ok = y1 < y2 && (y1..=y2).all(|y| sides_check(x1, y, b'-'));
            let bottom_ok = top_ok && (y1..=y2).all(|y| sides_check(x2, y, b'-'));
            let left_ok = bottom_ok && x1 < x2 && (x1..=x2).all(|x| sides_check(x, y1, b'|'));
            let right_ok = left_ok && (x1..=x2).all(|x| sides_check(x, y2, b'|'));

            if right_ok && lines[x1].as_bytes()[y2] == b'+' && lines[x2].as_bytes()[y1] == b'+' {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rows() {
        let input = &[];
        let output = count(input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn no_columns() {
        let input = &[""];
        let output = count(input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn no_rectangles() {
        let input = &[" "];
        let output = count(input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn one_rectangle() {
        #[rustfmt::skip]
        let input = &[
            "+-+",
            "| |",
            "+-+",
        ];
        let output = count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn two_rectangles_without_shared_parts() {
        #[rustfmt::skip]
        let input = &[
            "  +-+",
            "  | |",
            "+-+-+",
            "| |  ",
            "+-+  ",
        ];
        let output = count(input);
        let expected = 2;
        assert_eq!(output, expected);
    }

    #[test]
    fn five_rectangles_with_shared_parts() {
        #[rustfmt::skip]
        let input = &[
            "  +-+",
            "  | |",
            "+-+-+",
            "| | |",
            "+-+-+",
        ];
        let output = count(input);
        let expected = 5;
        assert_eq!(output, expected);
    }

    #[test]
    fn rectangle_of_height_1_is_counted() {
        #[rustfmt::skip]
        let input = &[
            "+--+",
            "+--+",
        ];
        let output = count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn rectangle_of_width_1_is_counted() {
        #[rustfmt::skip]
        let input = &[
            "++",
            "||",
            "++",
        ];
        let output = count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_1x1_square_is_counted() {
        #[rustfmt::skip]
        let input = &[
            "++",
            "++",
        ];
        let output = count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn only_complete_rectangles_are_counted() {
        #[rustfmt::skip]
        let input = &[
            "  +-+",
            "    |",
            "+-+-+",
            "| | -",
            "+-+-+",
        ];
        let output = count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn rectangles_can_be_of_different_sizes() {
        #[rustfmt::skip]
        let input = &[
            "+------+----+",
            "|      |    |",
            "+---+--+    |",
            "|   |       |",
            "+---+-------+",
        ];
        let output = count(input);
        let expected = 3;
        assert_eq!(output, expected);
    }

    #[test]
    fn corner_is_required_for_a_rectangle_to_be_complete() {
        #[rustfmt::skip]
        let input = &[
            "+------+----+",
            "|      |    |",
            "+------+    |",
            "|   |       |",
            "+---+-------+",
        ];
        let output = count(input);
        let expected = 2;
        assert_eq!(output, expected);
    }

    #[test]
    fn large_input_with_many_rectangles() {
        #[rustfmt::skip]
        let input = &[
            "+---+--+----+",
            "|   +--+----+",
            "+---+--+    |",
            "|   +--+----+",
            "+---+--+--+-+",
            "+---+--+--+-+",
            "+------+  | |",
            "          +-+",
        ];
        let output = count(input);
        let expected = 60;
        assert_eq!(output, expected);
    }

    #[test]
    fn rectangles_must_have_four_sides() {
        #[rustfmt::skip]
        let input = &[
            "+-+ +-+",
            "| | | |",
            "+-+-+-+",
            "  | |  ",
            "+-+-+-+",
            "| | | |",
            "+-+ +-+",
        ];
        let output = count(input);
        let expected = 5;
        assert_eq!(output, expected);
    }
}
