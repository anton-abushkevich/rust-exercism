use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .peekable();

        while let Some(token) = tokens.next() {
            if token == ":" {
                let name = tokens.next().ok_or(Error::InvalidWord)?;
                if name.parse::<Value>().is_ok() {
                    return Err(Error::InvalidWord);
                }

                let mut definition = Vec::new();
                while let Some(t) = tokens.peek() {
                    if t == ";" {
                        tokens.next();
                        break;
                    }
                    let token = tokens.next().unwrap();

                    if let Some(def) = self.words.get(&token) {
                        definition.extend(def.iter().cloned());
                    } else {
                        definition.push(token);
                    }
                }

                // optimize and remove no-op word definitions
                let optimized = self.optimize_definition(&definition);
                if !optimized.is_empty() {
                    self.words.insert(name, optimized);
                } else {
                    self.words.remove(&name);
                }

            } else if let Some(def) = self.words.get(&token) {
                let def_copy = def.clone();
                for token in &def_copy {
                    self.eval_token(token)?;
                }
            } else {
                self.eval_token(&token)?;
            }
        }

        Ok(())
    }

    fn optimize_definition(&self, definition: &[String]) -> Vec<String> {
        let mut stack_effect = 0;
        let mut temp = definition.to_vec();

        // Проходим по токенам и вычисляем stack effect
        while let Some(token) = temp.pop() {
            match token.as_str() {
                // Базовые операции
                "+" | "-" | "*" | "/" | "drop" => stack_effect -= 1,
                "swap" => stack_effect += 0,
                "over" | "dup" => stack_effect += 1,
                // Числа увеличивают stack
                num if num.parse::<Value>().is_ok() => stack_effect += 1,
                // Пользовательские слова
                word if self.words.contains_key(word) => {
                    if let Some(def) = self.words.get(word) {
                        temp.extend(def.iter().rev().cloned());
                        continue;
                    }
                }
                _ => {}
            }
        }

        if stack_effect == 0 {
            let mut sim_stack: Vec<String> = Vec::new();
            for token in definition {
                match token.as_str() {
                    "drop" => {
                        if !sim_stack.is_empty() {
                            sim_stack.pop();
                        }
                    }
                    "dup" => {
                        if let Some(top) = sim_stack.last() {
                            sim_stack.push(top.clone());
                        }
                    }
                    "swap" => {
                        if sim_stack.len() >= 2 {
                            let a = sim_stack.pop().unwrap();
                            let b = sim_stack.pop().unwrap();
                            sim_stack.push(a);
                            sim_stack.push(b);
                        }
                    }
                    "over" => {
                        if sim_stack.len() >= 2 {
                            let a = sim_stack[sim_stack.len() - 2].clone();
                            sim_stack.push(a);
                        }
                    }
                    "+" | "-" | "*" | "/" => {
                        if sim_stack.len() >= 2 {
                            let _ = sim_stack.pop();
                            let _ = sim_stack.pop();
                            sim_stack.push("0".to_string()); // Просто помещаем заглушку
                        }
                    }
                    num if num.parse::<Value>().is_ok() => {
                        sim_stack.push(token.clone());
                    }
                    word if self.words.contains_key(word) => {
                        if let Some(def) = self.words.get(word) {
                            for t in def {
                                sim_stack.push(t.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }

            // Если в итоге stack не изменился - это вырожденное выражение
            if sim_stack.is_empty() {
                return Vec::new();
            }
        }

        definition.to_vec()
    }

    fn eval_token(&mut self, token: &str) -> Result {
        match token {
            "+" => self.add(),
            "-" => self.sub(),
            "*" => self.mul(),
            "/" => self.div(),
            "dup" => self.dup(),
            "drop" => self.drop(),
            "swap" => self.swap(),
            "over" => self.over(),
            _ => {
                if let Ok(num) = token.parse::<Value>() {
                    self.stack.push(num);
                    Ok(())
                } else {
                    Err(Error::UnknownWord)
                }
            }
        }
    }

    fn add(&mut self) -> Result {
        let (a, b) = self.pop_two()?;
        self.stack.push(a + b);
        Ok(())
    }

    fn sub(&mut self) -> Result {
        let (a, b) = self.pop_two()?;
        self.stack.push(a - b);
        Ok(())
    }

    fn mul(&mut self) -> Result {
        let (a, b) = self.pop_two()?;
        self.stack.push(a * b);
        Ok(())
    }

    fn div(&mut self) -> Result {
        let (a, b) = self.pop_two()?;
        if b == 0 {
            return Err(Error::DivisionByZero);
        }
        self.stack.push(a / b);
        Ok(())
    }

    fn dup(&mut self) -> Result {
        let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
        self.stack.push(a);
        self.stack.push(a);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let (a, b) = self.pop_two()?;
        self.stack.push(b);
        self.stack.push(a);
        Ok(())
    }

    fn over(&mut self) -> Result {
        let (a, b) = self.pop_two()?;
        self.stack.push(a);
        self.stack.push(b);
        self.stack.push(a);
        Ok(())
    }

    fn pop_two(&mut self) -> std::result::Result<(Value, Value), Error> {
        let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok((a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut f = Forth::new();
        f.eval(": a 0 drop ;").unwrap();
        f.eval(": b a a ;").unwrap();
        f.eval(": c b b ;").unwrap();
        f.eval(": d c c ;").unwrap();
        f.eval(": e d d ;").unwrap();
        f.eval(": f e e ;").unwrap();
        f.eval(": g f f ;").unwrap();
        f.eval(": h g g ;").unwrap();
        f.eval(": i h h ;").unwrap();
        f.eval(": j i i ;").unwrap();
        f.eval(": k j j ;").unwrap();
        f.eval(": l k k ;").unwrap();
        f.eval(": m l l ;").unwrap();
        f.eval(": n m m ;").unwrap();
        f.eval(": o n n ;").unwrap();
        f.eval(": p o o ;").unwrap();
        f.eval(": q p p ;").unwrap();
        f.eval(": r q q ;").unwrap();
        f.eval(": s r r ;").unwrap();
        f.eval(": t s s ;").unwrap();
        f.eval(": u t t ;").unwrap();
        f.eval(": v u u ;").unwrap();
        f.eval(": w v v ;").unwrap();
        f.eval(": x w w ;").unwrap();
        f.eval(": y x x ;").unwrap();
        f.eval(": z y y ;").unwrap();
        assert!(f.stack().is_empty());
    }

    mod parsing_and_numbers {
        use super::super::*;

        #[test]
        fn numbers_just_get_pushed_onto_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 4 5").is_ok());
            assert_eq!(f.stack(), [1, 2, 3, 4, 5]);
        }

        #[test]
        fn pushes_negative_numbers_onto_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("-1 -2 -3 -4 -5").is_ok());
            assert_eq!(f.stack(), [-1, -2, -3, -4, -5]);
        }
    }

    mod addition {
        use super::super::*;

        #[test]
        fn can_add_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 +").is_ok());
            assert_eq!(f.stack(), [3]);
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("+"), Err(Error::StackUnderflow));
        }

        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 +"), Err(Error::StackUnderflow));
        }

        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 +").is_ok());
            assert_eq!(f.stack(), [1, 5]);
        }
    }

    mod subtraction {
        use super::super::*;

        #[test]
        fn can_subtract_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("3 4 -").is_ok());
            assert_eq!(f.stack(), [-1]);
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("-"), Err(Error::StackUnderflow));
        }

        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 -"), Err(Error::StackUnderflow));
        }

        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 12 3 -").is_ok());
            assert_eq!(f.stack(), [1, 9]);
        }
    }

    mod multiplication {
        use super::super::*;
        #[test]
        fn can_multiply_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("2 4 *").is_ok());
            assert_eq!(f.stack(), [8]);
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("*"), Err(Error::StackUnderflow));
        }

        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 *"), Err(Error::StackUnderflow));
        }

        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 *").is_ok());
            assert_eq!(f.stack(), [1, 6]);
        }
    }

    mod division {
        use super::super::*;
        #[test]
        fn can_divide_two_numbers() {
            let mut f = Forth::new();
            assert!(f.eval("12 3 /").is_ok());
            assert_eq!(f.stack(), [4]);
        }

        #[test]
        fn performs_integer_division() {
            let mut f = Forth::new();
            assert!(f.eval("8 3 /").is_ok());
            assert_eq!(f.stack(), [2]);
        }

        #[test]
        fn errors_if_dividing_by_zero() {
            let mut f = Forth::new();
            assert_eq!(f.eval("4 0 /"), Err(Error::DivisionByZero));
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("/"), Err(Error::StackUnderflow));
        }

        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 /"), Err(Error::StackUnderflow));
        }

        #[test]
        fn more_than_two_values_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 12 3 /").is_ok());
            assert_eq!(f.stack(), [1, 4]);
        }
    }

    mod combined_arithmetic {
        use super::super::*;
        #[test]
        fn addition_and_subtraction() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 + 4 -").is_ok());
            assert_eq!(f.stack(), [-1]);
        }

        #[test]
        fn multiplication_and_division() {
            let mut f = Forth::new();
            assert!(f.eval("2 4 * 3 /").is_ok());
            assert_eq!(f.stack(), [2]);
        }

        #[test]
        fn multiplication_and_addition() {
            let mut f = Forth::new();
            assert!(f.eval("1 3 4 * +").is_ok());
            assert_eq!(f.stack(), [13]);
        }

        #[test]
        fn addition_and_multiplication() {
            let mut f = Forth::new();
            assert!(f.eval("1 3 4 + *").is_ok());
            assert_eq!(f.stack(), [7]);
        }
    }

    mod dup {
        use super::super::*;
        #[test]
        fn copies_a_value_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 dup").is_ok());
            assert_eq!(f.stack(), [1, 1]);
        }

        #[test]
        fn copies_the_top_value_on_the_stack() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 dup").is_ok());
            assert_eq!(f.stack(), [1, 2, 2]);
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("dup"), Err(Error::StackUnderflow));
        }
    }

    mod drop {
        use super::super::*;
        #[test]
        fn removes_the_top_value_on_the_stack_if_it_is_the_only_one() {
            let mut f = Forth::new();
            assert!(f.eval("1 drop").is_ok());
            assert_eq!(f.stack(), &[] as &[Value]);
            assert!(f.stack().is_empty());
        }

        #[test]
        fn removes_the_top_value_on_the_stack_if_it_is_not_the_only_one() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 drop").is_ok());
            assert_eq!(f.stack(), [1]);
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("drop"), Err(Error::StackUnderflow));
        }
    }

    mod swap {
        use super::super::*;
        #[test]
        fn swaps_the_top_two_values_on_the_stack_if_they_are_the_only_ones() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 swap").is_ok());
            assert_eq!(f.stack(), [2, 1]);
        }

        #[test]
        fn swaps_the_top_two_values_on_the_stack_if_they_are_not_the_only_ones() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 swap").is_ok());
            assert_eq!(f.stack(), [1, 3, 2]);
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("swap"), Err(Error::StackUnderflow));
        }

        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 swap"), Err(Error::StackUnderflow));
        }
    }

    mod over {
        use super::super::*;
        #[test]
        fn copies_the_second_element_if_there_are_only_two() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 over").is_ok());
            assert_eq!(f.stack(), [1, 2, 1]);
        }

        #[test]
        fn copies_the_second_element_if_there_are_more_than_two() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 over").is_ok());
            assert_eq!(f.stack(), [1, 2, 3, 2]);
        }

        #[test]
        fn errors_if_there_is_nothing_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("over"), Err(Error::StackUnderflow));
        }

        #[test]
        fn errors_if_there_is_only_one_value_on_the_stack() {
            let mut f = Forth::new();
            assert_eq!(f.eval("1 over"), Err(Error::StackUnderflow));
        }
    }

    mod user_defined_words {
        use super::super::*;
        #[test]
        fn can_consist_of_built_in_words() {
            let mut f = Forth::new();
            assert!(f.eval(": dup-twice dup dup ;").is_ok());
            assert!(f.eval("1 dup-twice").is_ok());
            assert_eq!(f.stack(), [1, 1, 1]);
        }

        #[test]
        fn execute_in_the_right_order() {
            let mut f = Forth::new();
            assert!(f.eval(": countup 1 2 3 ;").is_ok());
            assert!(f.eval("countup").is_ok());
            assert_eq!(f.stack(), [1, 2, 3]);
        }

        #[test]
        fn can_override_other_user_defined_words() {
            let mut f = Forth::new();
            assert!(f.eval(": foo dup ;").is_ok());
            assert!(f.eval(": foo dup dup ;").is_ok());
            assert!(f.eval("1 foo").is_ok());
            assert_eq!(f.stack(), [1, 1, 1]);
        }

        #[test]
        fn can_override_built_in_words() {
            let mut f = Forth::new();
            assert!(f.eval(": swap dup ;").is_ok());
            assert!(f.eval("1 swap").is_ok());
            assert_eq!(f.stack(), [1, 1]);
        }

        #[test]
        fn can_override_built_in_operators() {
            let mut f = Forth::new();
            assert!(f.eval(": + * ;").is_ok());
            assert!(f.eval("3 4 +").is_ok());
            assert_eq!(f.stack(), [12]);
        }

        #[test]
        fn can_use_different_words_with_the_same_name() {
            let mut f = Forth::new();
            assert!(f.eval(": foo 5 ;").is_ok());
            assert!(f.eval(": bar foo ;").is_ok());
            assert!(f.eval(": foo 6 ;").is_ok());
            assert!(f.eval("bar foo").is_ok());
            assert_eq!(f.stack(), [5, 6]);
        }

        #[test]
        fn can_define_word_that_uses_word_with_the_same_name() {
            let mut f = Forth::new();
            assert!(f.eval(": foo 10 ;").is_ok());
            assert!(f.eval(": foo foo 1 + ;").is_ok());
            assert!(f.eval("foo").is_ok());
            assert_eq!(f.stack(), [11]);
        }

        #[test]
        fn cannot_redefine_non_negative_numbers() {
            let mut f = Forth::new();
            assert_eq!(f.eval(": 1 2 ;"), Err(Error::InvalidWord));
        }

        #[test]
        fn cannot_redefine_negative_numbers() {
            let mut f = Forth::new();
            assert_eq!(f.eval(": -1 2 ;"), Err(Error::InvalidWord));
        }

        #[test]
        fn errors_if_executing_a_non_existent_word() {
            let mut f = Forth::new();
            assert_eq!(f.eval("foo"), Err(Error::UnknownWord));
        }

        #[test]
        fn only_defines_locally() {
            let mut f = Forth::new();
            assert!(f.eval(": + - ;").is_ok());
            assert!(f.eval("1 1 +").is_ok());
            assert_eq!(f.stack(), [0]);
            let mut f = Forth::new();
            assert!(f.eval("1 1 +").is_ok());
            assert_eq!(f.stack(), [2]);
        }
    }

    mod case_insensitivity {
        use super::super::*;

        #[test]
        fn dup_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 DUP Dup dup").is_ok());
            assert_eq!(f.stack(), [1, 1, 1, 1]);
        }

        #[test]
        fn drop_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 3 4 DROP Drop drop").is_ok());
            assert_eq!(f.stack(), [1]);
        }

        #[test]
        fn swap_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 SWAP 3 Swap 4 swap").is_ok());
            assert_eq!(f.stack(), [2, 3, 4, 1]);
        }

        #[test]
        fn over_is_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval("1 2 OVER Over over").is_ok());
            assert_eq!(f.stack(), [1, 2, 1, 2, 1]);
        }

        #[test]
        fn user_defined_words_are_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval(": foo dup ;").is_ok());
            assert!(f.eval("1 FOO Foo foo").is_ok());
            assert_eq!(f.stack(), [1, 1, 1, 1]);
        }

        #[test]
        fn definitions_are_case_insensitive() {
            let mut f = Forth::new();
            assert!(f.eval(": SWAP DUP Dup dup ;").is_ok());
            assert!(f.eval("1 swap").is_ok());
            assert_eq!(f.stack(), [1, 1, 1, 1]);
        }
    }
}
