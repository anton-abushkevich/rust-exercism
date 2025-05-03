use std::iter::repeat;

pub fn get_diamond(c: char) -> Vec<String> {
    if !c.is_ascii_uppercase() { return Vec::new(); }
    let mut res= Vec::with_capacity(c as usize - 'A' as usize + 1);
    for i in 'A'..=c {
        let mut s = String::with_capacity(c as usize - 'A' as usize + 1);
        s.extend(repeat(' ').take(c as usize - i as usize));
        s.push(i);
        s.extend(repeat(' ').take(i as usize - 'A' as usize));
        res.push(mirror(&s.chars().collect::<Vec<_>>()).into_iter().collect());
    }
    mirror(&res)
}

fn mirror<T: Clone>(items: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(items.len() * 2 - 1);
    result.extend_from_slice(items);
    result.extend(items[..items.len()-1].iter().rev().cloned());
    result    
}

/* расставляет буквы по координатной сетке на ребрах алмаза. прочие координаты - пробелы.

fn get_char(order: i8, x: i8, y: i8) -> char {
	if x.abs() + y.abs() == order {
		(x.abs() as u8 + 'A' as u8) as _
	} else {
		' '
	}
}

pub fn get_diamond(c: char) -> Vec<String> {
	let order = c as i8 - 'A' as i8;
	(-order..=order).map(|y| (-order..=order).map(|x| get_char(order, x, y)).collect()).collect()
}

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degenerate_case_with_a_single_a_row() {
        assert_eq!(get_diamond('A'), vec!["A"]);
    }

    #[test]
    fn degenerate_case_with_no_row_containing_3_distinct_groups_of_spaces() {
        #[rustfmt::skip]
    assert_eq!(
        get_diamond('B'),
        vec![
            " A ",
            "B B",
            " A ",
        ]
    );
    }

    #[test]
    fn smallest_non_degenerate_case_with_odd_diamond_side_length() {
        #[rustfmt::skip]
    assert_eq!(
        get_diamond('C'),
        vec![
            "  A  ",
            " B B ",
            "C   C",
            " B B ",
            "  A  ",
        ]
    );
    }

    #[test]
    fn smallest_non_degenerate_case_with_even_diamond_side_length() {
        #[rustfmt::skip]
    assert_eq!(
        get_diamond('D'),
        vec![
            "   A   ",
            "  B B  ",
            " C   C ",
            "D     D",
            " C   C ",
            "  B B  ",
            "   A   ",
        ]
    );
    }

    #[test]
    fn largest_possible_diamond() {
        assert_eq!(
            get_diamond('Z'),
            vec![
                "                         A                         ",
                "                        B B                        ",
                "                       C   C                       ",
                "                      D     D                      ",
                "                     E       E                     ",
                "                    F         F                    ",
                "                   G           G                   ",
                "                  H             H                  ",
                "                 I               I                 ",
                "                J                 J                ",
                "               K                   K               ",
                "              L                     L              ",
                "             M                       M             ",
                "            N                         N            ",
                "           O                           O           ",
                "          P                             P          ",
                "         Q                               Q         ",
                "        R                                 R        ",
                "       S                                   S       ",
                "      T                                     T      ",
                "     U                                       U     ",
                "    V                                         V    ",
                "   W                                           W   ",
                "  X                                             X  ",
                " Y                                               Y ",
                "Z                                                 Z",
                " Y                                               Y ",
                "  X                                             X  ",
                "   W                                           W   ",
                "    V                                         V    ",
                "     U                                       U     ",
                "      T                                     T      ",
                "       S                                   S       ",
                "        R                                 R        ",
                "         Q                               Q         ",
                "          P                             P          ",
                "           O                           O           ",
                "            N                         N            ",
                "             M                       M             ",
                "              L                     L              ",
                "               K                   K               ",
                "                J                 J                ",
                "                 I               I                 ",
                "                  H             H                  ",
                "                   G           G                   ",
                "                    F         F                    ",
                "                     E       E                     ",
                "                      D     D                      ",
                "                       C   C                       ",
                "                        B B                        ",
                "                         A                         ",
            ]
        );
    }
}