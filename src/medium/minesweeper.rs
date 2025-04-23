pub fn _annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![]
    }
    if minefield[0].is_empty() {
        return vec!("".to_string())
    }
    let rows = minefield.len() as i8;
    let cols = minefield[0].len() as i8;
    let mut result = Vec::with_capacity(rows as usize);

    for i in 0..rows {
        let sbytes: &[u8] = minefield[i as usize].as_bytes();
        let mut row = String::with_capacity(cols as usize);
        for j in 0..cols {
            if sbytes[j as usize] == b'*' {
                row.push('*');
                continue; // skip bombs
            }
            let points: [_; 8] = [
                (i-1, j-1), (i-1, j), (i-1, j+1),
                (i, j-1), (i, j+1),
                (i+1, j-1), (i+1, j), (i+1, j+1)];
            let mut count: u8 = 0;
            for x in points {
                if x.0 < 0 || x.0 >= rows || x.1 < 0 || x.1 >= cols { continue }
                if minefield[x.0 as usize].as_bytes()[x.1 as usize] == b'*' {
                    count += 1;
                }
            }
            if count > 0 {
                row.push_str(&count.to_string());
            } else {
                row.push(' ');
            }
        }
        result.push(row);
    }

    result
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }
    if minefield[0].is_empty() {
        return vec!("".to_string())
    }

    let rows = minefield.len();
    let cols = minefield[0].len();

    (0..rows).map(|i| {
        minefield[i].chars().enumerate().map(|(j, c)| {
            if c == '*' {
                return '*';
            }

            let neighbors = (-1..=1).flat_map(|di| (-1..=1).map(move |dj| (di, dj)));
            let count = neighbors
                .filter(|&(di, dj)| !(di == 0 && dj == 0))
                .filter_map(|(di, dj)| {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                        Some((ni as usize, nj as usize))
                    } else {
                        None
                    }
                })
                .filter(|&(ni, nj)| minefield[ni].as_bytes()[nj] == b'*')
                .count();

            match count {
                0 => ' ',
                n => char::from_digit(n as u32, 10).unwrap()
            }
        }).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rows() {
        let input = &[];
        let expected: &[&str] = &[];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_columns() {
        let input = &[""];
        let expected = &[""];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_mines() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            "   ",
            "   ",
            "   ",
        ], &[
            "   ",
            "   ",
            "   ",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn minefield_with_only_mines() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            "***",
            "***",
            "***",
        ], &[
            "***",
            "***",
            "***",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            "   ",
            " * ",
            "   ",
        ], &[
            "111",
            "1*1",
            "111",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            "***",
            "* *",
            "***",
        ], &[
            "***",
            "*8*",
            "***",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn horizontal_line() {
        let input = &[" * * "];
        let expected = &["1*2*1"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn horizontal_line_mines_at_edges() {
        let input = &["*   *"];
        let expected = &["*1 1*"];
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            " ",
            "*",
            " ",
            "*",
            " ",
        ], &[
            "1",
            "*",
            "2",
            "*",
            "1",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            "*",
            " ",
            " ",
            " ",
            "*",
        ], &[
            "*",
            "1",
            " ",
            "1",
            "*",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn cross() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            "  *  ",
            "  *  ",
            "*****",
            "  *  ",
            "  *  ",
        ], &[
            " 2*2 ",
            "25*52",
            "*****",
            "25*52",
            " 2*2 ",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_minefield() {
        #[rustfmt::skip]
            let (input, expected) = (&[
            " *  * ",
            "  *   ",
            "    * ",
            "   * *",
            " *  * ",
            "      ",
        ], &[
            "1*22*1",
            "12*322",
            " 123*2",
            "112*4*",
            "1*22*2",
            "111111",
        ]);
        let actual = annotate(input);
        assert_eq!(actual, expected);
    }
}
