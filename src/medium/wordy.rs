pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") || !command.ends_with("?") { return None }
    
    let mut command = command[8..command.len() - 1].split_whitespace();
    let operand = parse_operand(command.next())?;
    let mut res = operand;

    while let Some(operator) = command.next() {
        match operator {
            "plus" => res += parse_operand(command.next())?,
            "minus" => res -= parse_operand(command.next())?,
            "multiplied" => {
                if command.next()? != "by" { return None; }
                res *= parse_operand(command.next())?;
            },
            "divided" => {
                if command.next()? != "by" { return None; }
                res /= parse_operand(command.next())?;
            },
            "raised" => {
                if command.next()? != "to" { return None; }
                if command.next()? != "the" { return None; }
                let operand_th = command.next()?;
                if !operand_th.ends_with("th") && !operand_th.ends_with("nd") &&
                    !operand_th.ends_with("st") && !operand_th.ends_with("rd") {
                    return None;
                }
                let x = &operand_th[0..operand_th.len() - 2];
                let operand = parse_operand(Some(x))?;
                if operand < 0 {
                    return None;
                }
                if command.next()? != "power" { return None; }
                res = res.pow(operand as u32);
            }
            _ => return None,
        }
    }
    
    Some(res)
}

fn parse_operand(s: Option<&str>) -> Option<i32> {
    s?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_a_number() {
        let input = "What is 5?";
        let output = answer(input);
        let expected = Some(5);
        assert_eq!(output, expected);
    }

    #[test]
    fn addition() {
        let input = "What is 1 plus 1?";
        let output = answer(input);
        let expected = Some(2);
        assert_eq!(output, expected);
    }

    #[test]
    fn more_addition() {
        let input = "What is 53 plus 2?";
        let output = answer(input);
        let expected = Some(55);
        assert_eq!(output, expected);
    }

    #[test]
    fn addition_with_negative_numbers() {
        let input = "What is -1 plus -10?";
        let output = answer(input);
        let expected = Some(-11);
        assert_eq!(output, expected);
    }

    #[test]
    fn large_addition() {
        let input = "What is 123 plus 45678?";
        let output = answer(input);
        let expected = Some(45801);
        assert_eq!(output, expected);
    }

    #[test]
    fn subtraction() {
        let input = "What is 4 minus -12?";
        let output = answer(input);
        let expected = Some(16);
        assert_eq!(output, expected);
    }

    #[test]
    fn multiplication() {
        let input = "What is -3 multiplied by 25?";
        let output = answer(input);
        let expected = Some(-75);
        assert_eq!(output, expected);
    }

    #[test]
    fn division() {
        let input = "What is 33 divided by -3?";
        let output = answer(input);
        let expected = Some(-11);
        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_additions() {
        let input = "What is 1 plus 1 plus 1?";
        let output = answer(input);
        let expected = Some(3);
        assert_eq!(output, expected);
    }

    #[test]
    fn addition_and_subtraction() {
        let input = "What is 1 plus 5 minus -2?";
        let output = answer(input);
        let expected = Some(8);
        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_subtraction() {
        let input = "What is 20 minus 4 minus 13?";
        let output = answer(input);
        let expected = Some(3);
        assert_eq!(output, expected);
    }

    #[test]
    fn subtraction_then_addition() {
        let input = "What is 17 minus 6 plus 3?";
        let output = answer(input);
        let expected = Some(14);
        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_multiplication() {
        let input = "What is 2 multiplied by -2 multiplied by 3?";
        let output = answer(input);
        let expected = Some(-12);
        assert_eq!(output, expected);
    }

    #[test]
    fn addition_and_multiplication() {
        let input = "What is -3 plus 7 multiplied by -2?";
        let output = answer(input);
        let expected = Some(-8);
        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_division() {
        let input = "What is -12 divided by 2 divided by -3?";
        let output = answer(input);
        let expected = Some(2);
        assert_eq!(output, expected);
    }

    #[test]
    fn unknown_operation() {
        let input = "What is 52 cubed?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn non_math_question() {
        let input = "Who is the President of the United States?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn reject_problem_missing_an_operand() {
        let input = "What is 1 plus?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn reject_problem_with_no_operands_or_operators() {
        let input = "What is?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn reject_two_operations_in_a_row() {
        let input = "What is 1 plus plus 2?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn reject_two_numbers_in_a_row() {
        let input = "What is 1 plus 2 1?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn reject_postfix_notation() {
        let input = "What is 1 2 plus?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn reject_prefix_notation() {
        let input = "What is plus 1 2?";
        let output = answer(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn exponential() {
        let input = "What is 2 raised to the 5th power?";
        let output = answer(input);
        let expected = Some(32);
        assert_eq!(output, expected);
    }

    #[test]
    fn addition_and_exponential() {
        let input = "What is 1 plus 2 raised to the 2nd power?";
        let output = answer(input);
        let expected = Some(9);
        assert_eq!(output, expected);
    }
}