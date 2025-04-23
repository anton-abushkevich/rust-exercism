pub fn _reply_my(message: &str) -> &str {
    if message.trim() == "" {
        return "Fine. Be that way!";
    }

    let bytes = message.trim().as_bytes();
    let lowercases = 97..=122;
    let uppecases = 65..=90;
    let question = bytes.ends_with(&[63]);
    let has_lowercase = bytes.iter().any(|b| lowercases.contains(b));
    let has_uppercase = bytes.iter().any(|b| uppecases.contains(b));

    if question && !has_lowercase && has_uppercase {
        return "Calm down, I know what I'm doing!"
    }
    if question {
        return "Sure."
    }
    if !has_lowercase && has_uppercase {
        return "Whoa, chill out!"
    }
    "Whatever."
}

fn is_yelling(message: &str) -> bool {
    let have_letters: bool = message.chars().any(char::is_alphabetic);
    have_letters && message.to_uppercase() == message
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question() {
        assert_eq!(reply("ajhsdfg?"), "Sure.");
    }
    #[test]
    fn yell() {
        assert_eq!(reply("HHJDS AKSJD"), "Whoa, chill out!");
    }
    #[test]
    fn yell_question() {
        assert_eq!(reply("BVXZNM GDAHK?"), "Calm down, I know what I'm doing!");
    }
    #[test]
    fn silence() {
        assert_eq!(reply(""), "Fine. Be that way!");
    }
    #[test]
    fn other() {
        assert_eq!(reply("csdf"), "Whatever.");
    }
}