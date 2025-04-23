pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    if n % 3 == 0 {
        s.push_str("Pling");
    }
    if n % 5 == 0 {
        s.push_str("Plang");
    }
    if n % 7 == 0 {
        s.push_str("Plong");
    }
    if s.is_empty() {
        return n.to_string()
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let input = 105;
        let output = raindrops(input);
        let expected = "PlingPlangPlong";
        assert_eq!(output, expected);
    }
}
