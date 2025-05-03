pub fn _build_proverb_my(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.len() == 0 {
        return proverb;
    }
    let mut i = 1;
    while i < list.len() {
        proverb.push_str(format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]).as_str(),);
        i += 1;
    }
    proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());
    proverb
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(std::iter::once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_pieces() {
        let input = &[];
        let output = build_proverb(input);
        let expected = String::new();
        assert_eq!(output, expected);
    }
    #[test]
    fn one_piece() {
        let input = &["nail"];
        let output = build_proverb(input);
        let expected: String = ["And all for the want of a nail."].join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn two_pieces() {
        let input = &["nail", "shoe"];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn three_pieces() {
        let input = &["nail", "shoe", "horse"];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "And all for the want of a nail.",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn full_proverb() {
        let input = &[
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "For want of a horse the rider was lost.",
            "For want of a rider the message was lost.",
            "For want of a message the battle was lost.",
            "For want of a battle the kingdom was lost.",
            "And all for the want of a nail.",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }
    #[test]
    fn four_pieces_modernized() {
        let input = &["pin", "gun", "soldier", "battle"];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a pin the gun was lost.",
            "For want of a gun the soldier was lost.",
            "For want of a soldier the battle was lost.",
            "And all for the want of a pin.",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }
}
