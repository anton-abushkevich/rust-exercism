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
    fn check() {
        let list = [
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ];
        println!("{}", build_proverb(&list));
    }
}
