use std::collections::HashMap;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let mut degrees = HashMap::new();
    for &(a, b) in input {
        *degrees.entry(a).or_insert(0) += 1;
        *degrees.entry(b).or_insert(0) += 1;
    }
    if degrees.values().any(|&d| d % 2 != 0) {
        return None;
    }

    let mut used = vec![false; input.len()];
    let mut path = Vec::new();
    find_chain(input, &mut used, &mut path, input[0].0)
}

fn find_chain(
    input: &[(u8, u8)],
    used: &mut Vec<bool>,
    path: &mut Vec<(u8, u8)>,
    current: u8,
) -> Option<Vec<(u8, u8)>> {
    if used.iter().all(|&u| u) {
        return Some(path.clone());
    }

    for i in 0..input.len() {
        if !used[i] {
            let (a, b) = input[i];
            if a == current || b == current {
                used[i] = true;
                path.push(if a == current { (a, b) } else { (b, a) });
                let next = if a == current { b } else { a };

                if let Some(result) = find_chain(input, used, path, next) {
                    return Some(result);
                }

                path.pop();
                used[i] = false;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_empty_output() {
        let input = &[];
        let output = chain(input);
        assert!(output.is_some());
        assert_correct(input, output.unwrap());
    }

    #[test]
    fn singleton_input_singleton_output() {
        let input = &[(1, 1)];
        let output = chain(input);
        assert!(output.is_some());
        assert_correct(input, output.unwrap());
    }

    #[test]
    fn singleton_that_can_t_be_chained() {
        let input = &[(1, 2)];
        assert!(chain(input).is_none());
    }

    #[test]
    fn three_elements() {
        let input = &[(1, 2), (3, 1), (2, 3)];
        let output = chain(input);
        assert!(output.is_some());
        assert_correct(input, output.unwrap());
    }

    #[test]
    fn can_reverse_dominoes() {
        let input = &[(1, 2), (1, 3), (2, 3)];
        let output = chain(input);
        assert!(output.is_some());
        assert_correct(input, output.unwrap());
    }

    #[test]
    fn can_t_be_chained() {
        let input = &[(1, 2), (4, 1), (2, 3)];
        assert!(chain(input).is_none());
    }

    #[test]
    fn disconnected_simple() {
        let input = &[(1, 1), (2, 2)];
        assert!(chain(input).is_none());
    }

    #[test]
    fn disconnected_double_loop() {
        let input = &[(1, 2), (2, 1), (3, 4), (4, 3)];
        assert!(chain(input).is_none());
    }

    #[test]
    fn disconnected_single_isolated() {
        let input = &[(1, 2), (2, 3), (3, 1), (4, 4)];
        assert!(chain(input).is_none());
    }

    #[test]
    fn need_backtrack() {
        let input = &[(1, 2), (2, 3), (3, 1), (2, 4), (2, 4)];
        let output = chain(input);
        assert!(output.is_some());
        assert_correct(input, output.unwrap());
    }

    #[test]
    fn separate_loops() {
        let input = &[(2, 2), (1, 2), (2, 3), (3, 1), (1, 1), (3, 3)];
        let output = chain(input);
        assert!(output.is_some());
        assert_correct(input, output.unwrap());
    }

    #[test]
    fn nine_elements() {
        let input = &[
            (1, 2),
            (5, 3),
            (3, 1),
            (1, 2),
            (2, 4),
            (1, 6),
            (2, 3),
            (3, 4),
            (5, 6),
        ];
        let output = chain(input);
        assert!(output.is_some());
        assert_correct(input, output.unwrap());
    }

    #[test]
    fn separate_three_domino_loops() {
        let input = &[(1, 2), (2, 3), (3, 1), (4, 5), (5, 6), (6, 4)];
        assert!(chain(input).is_none());
    }

    type Domino = (u8, u8);
    fn assert_correct(input: &[Domino], output: Vec<Domino>) {
        if input.len() != output.len() {
            panic!("Length mismatch for input {input:?}, output {output:?}");
        } else if input.is_empty() {
            // and thus output.is_empty()
            return;
        }

        let mut output_sorted = output
            .iter()
            .map(|&d| normalize(d))
            .collect::<Vec<Domino>>();
        output_sorted.sort_unstable();
        let mut input_sorted = input.iter().map(|&d| normalize(d)).collect::<Vec<Domino>>();
        input_sorted.sort_unstable();
        if input_sorted != output_sorted {
            panic!("Domino mismatch for input {input:?}, output {output:?}");
        }

        // both input and output have at least 1 element
        // This essentially puts the first element after the last one, thereby making it
        // easy to check whether the domino chains "wraps around".
        {
            let mut n = output[0].1;
            let iter = output.iter().skip(1).chain(output.iter().take(1));
            for &(first, second) in iter {
                if n != first {
                    panic!("Chaining failure for input {input:?}, output {output:?}")
                }

                n = second
            }
        }
    }

    fn normalize(d: Domino) -> Domino {
        match d {
            (m, n) if m > n => (n, m),
            (m, n) => (m, n),
        }
    }
}
