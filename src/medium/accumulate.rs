pub fn map<F, T, R>(input: Vec<T>, mut function: F) -> Vec<R>
where
    F: FnMut(T) -> R
{
    let mut res = Vec::with_capacity(input.len());
    for el in input {
        res.push(function(el));
    }
    res
    
    // or simply input.into_iter().map(function).collect()
    // and then no need to mark function as mut
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(x: i32) -> i32 {
        x * x
    }

    #[test]
    fn accumulate_empty() {
        let input = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(map(input, square), expected);
    }

    #[test]
    fn func_single() {
        let input = vec![2];
        let expected = vec![4];
        assert_eq!(map(input, square), expected);
    }

    #[test]
    fn accumulate_squares() {
        let input = vec![1, 2, 3];
        let expected = vec![1, 4, 9];
        assert_eq!(map(input, square), expected);
    }

    #[test]
    fn closure() {
        let input = vec![2, 3, 4, 5];
        let expected = vec![4, 9, 16, 25];
        assert_eq!(map(input, |x| x * x), expected);
    }

    #[test]
    fn closure_floats() {
        let input = vec![2.0, 3.0, 4.0, 5.0];
        let expected = vec![4.0, 9.0, 16.0, 25.0];
        assert_eq!(map(input, |x| x * x), expected);
    }

    #[test]
    fn strings() {
        let input = vec!["1".to_string(), "2".into(), "3".into()];
        let expected = vec!["11".to_string(), "22".into(), "33".into()];
        assert_eq!(map(input, |s| s.repeat(2)), expected);
    }

    #[test]
    fn accumulate_upcases() {
        let input = vec!["Hello", "world"];
        let expected = vec!["HELLO", "WORLD"];
        assert_eq!(map(input, str::to_uppercase), expected);
    }

    #[test]
    fn accumulate_reversed_strings() {
        let input = vec!["the", "quick", "brown", "fox", "etc"];
        let expected = vec!["eht", "kciuq", "nworb", "xof", "cte"];
        let reverse = |s: &str| s.chars().rev().collect::<String>();
        assert_eq!(map(input, reverse), expected);
    }

    #[test]
    fn accumulate_recursively() {
        let input = vec!["a", "b", "c"];
        let expected = vec![
            vec!["a1", "a2", "a3"],
            vec!["b1", "b2", "b3"],
            vec!["c1", "c2", "c3"],
        ];
        assert_eq!(
            map(input, |x| map(vec!["1", "2", "3"], |y| [x, y].join(""))),
            expected
        );
    }

    #[test]
    fn change_in_type() {
        let input: Vec<&str> = vec!["1", "2", "3"];
        let expected: Vec<String> = vec!["1".into(), "2".into(), "3".into()];
        assert_eq!(map(input, |s| s.to_string()), expected);
    }

    #[test]
    fn mutating_closure() {
        let mut counter = 0;
        let input = vec![-2, 3, 4, -5];
        let expected = vec![2, 3, 4, 5];
        let result = map(input, |x: i64| {
            counter += 1;
            x.abs()
        });
        assert_eq!(result, expected);
        assert_eq!(counter, 4);
    }

    #[test]
    fn minimal_bounds_on_input_and_output() {
        // must be able to accept arbitrary input and output types
        struct Foo;
        struct Bar;
        map(vec![Foo], |_| Bar);
    }
}
