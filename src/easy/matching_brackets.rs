fn _brackets_are_balanced(string: &str) -> bool {
    let openings = ['{', '[', '('];
    let closings = ['}', ']', ')'];
    let mut brackets: Vec<char> = Vec::new();
    for c in string.chars() {
        if openings.contains(&c) {
            brackets.push(c);
        } else if let Some(i) = closings.iter().position(|x| x == &c) {
           if let Some(bracket) = brackets.last() {
               if *bracket == openings[i] {
                   brackets.pop();
               } else {
                   return false
               }
           } else {
               return false;
           }
       }
    }
    brackets.is_empty()
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();
    for c in string.chars() {
        match c {
            '{' => brackets.push('}'),
            '[' => brackets.push(']'),
            '(' => brackets.push(')'),
            '}' | ']' | ')' if brackets.pop() != Some(c) => return false,
            _ => (),
        }
    }
    brackets.is_empty()
}

#[test]
fn paired_square_brackets() {
    assert!(brackets_are_balanced("[]"));
    assert!(!brackets_are_balanced("{(}"));
    assert!(brackets_are_balanced("[asd{asda(as)asd}]"));
}
