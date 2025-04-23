use std::collections::HashSet;

pub fn _anagrams_for<'a>(word: &'a str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let perms = _permutations(word);
    possible_anagrams.iter()
        .filter(|&&s| perms.iter().any(|a| a == s))
        .copied().collect()
}

fn _permutations(word: &str) -> Vec<String> {
    let mut chars: Vec<char> = word.chars().collect();
    let mut result = Vec::new();
    chars.sort();

    fn backtrack(
        chars: &Vec<char>,
        path: &mut Vec<char>,
        used: &mut Vec<bool>,
        result: &mut Vec<String>,
    ) {
        if path.len() == chars.len() {
            result.push(path.iter().collect());
            return;
        }

        for i in 0..chars.len() {
            if used[i] || (i > 0 && chars[i] == chars[i - 1] && !used[i - 1]) {
                continue;
            }
            used[i] = true;
            path.push(chars[i]);
            backtrack(chars, path, used, result);
            path.pop();
            used[i] = false;
        }
    }

    backtrack(&chars, &mut Vec::new(), &mut vec![false; chars.len()], &mut result);
    result
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let normalized_word = normalize(word);
    possible_anagrams
        .iter()
        .filter(|&&s| s.to_lowercase() != word.to_lowercase() && normalize(s) == normalized_word)
        .copied()
        .collect()
}

fn normalize(s: &str) -> String {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

#[test]
fn no_matches() {
    assert_eq!(anagrams_for("diaper", &["hello", "world", "zombies", "pants"]),
               HashSet::from_iter([]));
    assert_eq!(anagrams_for("solemn", &["lemons", "cherry", "melons"]),
               HashSet::from_iter(["lemons", "melons"]));
}
