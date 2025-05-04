/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    struct AppendIterator<I, J> {
        a: I,
        b: J,
    }

    impl<I, J> Iterator for AppendIterator<I, J>
    where
        I: Iterator,
        J: Iterator<Item = I::Item>,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            match self.a.next() {
                Some(item) => Some(item),
                None => self.b.next(),
            }
        }
    }    
    
    AppendIterator { a, b }
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    struct ConcatIterator<I: Iterator> {
        outer_iter: I,
        inner_iter: Option<I::Item>,
    }

    impl<I> Iterator for ConcatIterator<I>
    where
        I: Iterator,
        I::Item: Iterator,
    {
        type Item = <I::Item as Iterator>::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if let Some(inner) = &mut self.inner_iter {
                    if let Some(item) = inner.next() {
                        return Some(item);
                    }
                }
                self.inner_iter = self.outer_iter.next();
                self.inner_iter.as_ref()?;
            }
        }
    }
    
    ConcatIterator {
        outer_iter: nested_iter,
        inner_iter: None,
    }
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    struct FilterIterator<I, F> {
        iter: I,
        predicate: F
    }

    impl<I, F> Iterator for FilterIterator<I, F>
    where
        I: Iterator,
        F: Fn(&I::Item) -> bool,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            // clippy wants me to use find() here, but I... must... resist...
            for i in self.iter.by_ref() {
                if (self.predicate)(&i) {
                    return Some(i);
                }
            }
            None
        }
    }    
    
    FilterIterator { iter, predicate }
}

pub fn length<I: Iterator>(iter: I) -> usize {
    foldl(iter, 0, |acc, _| acc + 1)
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    struct MapIterator<I, F> {
        iter: I,
        function: F
    }
    
    impl<I, F, U> Iterator for MapIterator<I, F>
    where
        I: Iterator,
        F: Fn(I::Item) -> U,    
    {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(i) = self.iter.next() {
                return Some((self.function)(i));
            }
            None
        }
    }
    
    MapIterator { iter, function }
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut res: U = initial;
    for item in iter {
        res = function(res, item);
    }    
    res
}

pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut res: U = initial;
    for i in reverse(iter) {
        res = function(res, i);
    }
    res
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    struct ReverseIterator<I: DoubleEndedIterator>(I);

    impl<I: DoubleEndedIterator> Iterator for ReverseIterator<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next_back()
        }
    }

    ReverseIterator(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod append {
        use super::*;
        #[test]
        fn empty_lists() {
            let list1 = vec![0i32; 0].into_iter();
            let list2 = vec![0i32; 0].into_iter();
            let output = append(list1, list2);
            let expected: Vec<i32> = vec![];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn list_to_empty_list() {
            let list1 = vec![0i32; 0].into_iter();
            let list2 = vec![1, 2, 3, 4].into_iter();
            let output = append(list1, list2);
            let expected = vec![1, 2, 3, 4];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn empty_list_to_list() {
            let list1 = vec![1, 2, 3, 4].into_iter();
            let list2 = vec![0i32; 0].into_iter();
            let output = append(list1, list2);
            let expected = vec![1, 2, 3, 4];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn non_empty_lists() {
            let list1 = vec![1, 2].into_iter();
            let list2 = vec![2, 3, 4, 5].into_iter();
            let output = append(list1, list2);
            let expected = vec![1, 2, 2, 3, 4, 5];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }
    }

    #[allow(clippy::zero_repeat_side_effects)]
    mod concat {
        use super::*;
        #[test]
        fn empty_list() {
            let lists = vec![vec![0i32; 0]; 0].into_iter().map(Vec::into_iter);
            let output = concat(lists);
            let expected: Vec<i32> = vec![];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn list_of_lists() {
            let lists = vec![vec![1, 2], vec![3], vec![], vec![4, 5, 6]]
                .into_iter()
                .map(Vec::into_iter);
            let output = concat(lists);
            let expected = vec![1, 2, 3, 4, 5, 6];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn list_of_nested_lists() {
            let lists = vec![
                vec![vec![1], vec![2]],
                vec![vec![3]],
                vec![vec![]],
                vec![vec![4, 5, 6]],
            ]
                .into_iter()
                .map(Vec::into_iter);
            let output = concat(lists);
            let expected = vec![vec![1], vec![2], vec![3], vec![], vec![4, 5, 6]];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }
    }

    mod filter {
        use super::*;
        #[test]
        fn empty_list() {
            let list = vec![0i32; 0].into_iter();
            let output = filter(list, |x| x % 2 == 1);
            let expected: Vec<i32> = vec![];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn non_empty_list() {
            let list = vec![1, 2, 3, 5].into_iter();
            let output = filter(list, |x| x % 2 == 1);
            let expected = vec![1, 3, 5];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }
    }

    mod length {
        use super::*;
        #[test]
        fn empty_list() {
            let list = vec![0i32; 0].into_iter();
            let output = length(list);
            let expected = 0;
            assert_eq!(output, expected);
        }

        #[test]
        fn non_empty_list() {
            let list = vec![1, 2, 3, 4].into_iter();
            let output = length(list);
            let expected = 4;
            assert_eq!(output, expected);
        }
    }

    mod map {
        use super::*;
        #[test]
        fn empty_list() {
            let list = vec![0i32; 0].into_iter();
            let output = map(list, |x| x + 1);
            let expected: Vec<i32> = vec![];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn non_empty_list() {
            let list = vec![1, 3, 5, 7].into_iter();
            let output = map(list, |x| x + 1);
            let expected = vec![2, 4, 6, 8];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }
    }

    mod foldl {
        use super::*;
        #[test]
        fn empty_list() {
            let list = vec![0.0f64; 0].into_iter();
            let initial = 2.0;
            let output = foldl(list, initial, |acc, el| el * acc);
            let expected = 2.0;
            assert_eq!(output, expected);
        }

        #[test]
        fn direction_independent_function_applied_to_non_empty_list() {
            let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
            let initial = 5.0;
            let output = foldl(list, initial, |acc, el| el + acc);
            let expected = 15.0;
            assert_eq!(output, expected);
        }

        #[test]
        fn direction_dependent_function_applied_to_non_empty_list() {
            let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
            let initial = 24.0;
            let output = foldl(list, initial, |acc, el| el / acc);
            let expected = 64.0;
            assert_eq!(output, expected);
        }
    }

    mod foldr {
        use super::*;
        #[test]
        fn empty_list() {
            let list = vec![0.0f64; 0].into_iter();
            let initial = 2.0;
            let output = foldr(list, initial, |acc, el| el * acc);
            let expected = 2.0;
            assert_eq!(output, expected);
        }

        #[test]
        fn direction_independent_function_applied_to_non_empty_list() {
            let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
            let initial = 5.0;
            let output = foldr(list, initial, |acc, el| el + acc);
            let expected = 15.0;
            assert_eq!(output, expected);
        }

        #[test]
        fn direction_dependent_function_applied_to_non_empty_list() {
            let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
            let initial = 24.0;
            let output = foldr(list, initial, |acc, el| el / acc);
            let expected = 9.0;
            assert_eq!(output, expected);
        }
    }

    mod reverse {
        use super::*;
        #[test]
        fn empty_list() {
            let list = vec![0i32; 0].into_iter();
            let output = reverse(list);
            let expected: Vec<i32> = vec![];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn non_empty_list() {
            let list = vec![1, 3, 5, 7].into_iter();
            let output = reverse(list);
            let expected = vec![7, 5, 3, 1];
            assert_eq!(output.collect::<Vec<_>>(), expected);
        }

        #[test]
        fn list_of_lists_is_not_flattened() {
            let list = vec![vec![1, 2], vec![3], vec![], vec![4, 5, 6]]
                .into_iter()
                .map(Vec::into_iter);
            let output = reverse(list);
            let expected = vec![vec![4, 5, 6], vec![], vec![3], vec![1, 2]];
            assert_eq!(
                output
                    .map(|subiter| subiter.collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }
}