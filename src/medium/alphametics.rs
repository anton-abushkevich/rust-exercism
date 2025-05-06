use std::collections::{HashMap, HashSet};

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = puzzle.split_once("==").expect("Invalid equation");
    let left_parts: Vec<&str> = left.split('+').map(|s| s.trim()).collect();
    let right = right.trim();

    let all_letters: Vec<char> = puzzle
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let first_letters: HashSet<char> = left_parts
        .iter()
        .chain(&[right])
        .map(|word| word.chars().next().unwrap())
        .collect();

    let mut letter_to_digit = HashMap::new();
    let mut used_digits = [false; 10];

    backtrack(
        &all_letters,
        &left_parts,
        right,
        &first_letters,
        &mut letter_to_digit,
        &mut used_digits,
        0,
    )
}

fn backtrack(
    letters: &[char],
    left_parts: &[&str],
    right: &str,
    first_letters: &HashSet<char>,
    letter_to_digit: &mut HashMap<char, u8>,
    used_digits: &mut [bool; 10],
    depth: usize,
) -> Option<HashMap<char, u8>> {
    if depth == letters.len() {
        if is_valid(left_parts, right, letter_to_digit) {
            return Some(letter_to_digit.clone());
        }
        return None;
    }

    let current_char = letters[depth];
    let is_first_letter = first_letters.contains(&current_char);

    for digit in 0..10 {
        if (digit == 0 && is_first_letter) || used_digits[digit as usize] {
            continue;
        }

        used_digits[digit as usize] = true;
        letter_to_digit.insert(current_char, digit);

        if let Some(solution) = backtrack(
            letters,
            left_parts,
            right,
            first_letters,
            letter_to_digit,
            used_digits,
            depth + 1,
        ) {
            return Some(solution);
        }

        letter_to_digit.remove(&current_char);
        used_digits[digit as usize] = false;
    }

    None
}

fn is_valid(left_parts: &[&str], right: &str, letter_to_digit: &HashMap<char, u8>) -> bool {
    let right_value = match word_to_value(right, letter_to_digit) {
        Some(v) => v,
        None => return false,
    };

    let mut left_sum = 0;
    for word in left_parts {
        match word_to_value(word, letter_to_digit) {
            Some(v) => left_sum += v,
            None => return false,
        }
    }

    left_sum == right_value
}

fn word_to_value(word: &str, letter_to_digit: &HashMap<char, u8>) -> Option<u64> {
    let mut value = 0;
    for c in word.chars() {
        value = value * 10 + (*letter_to_digit.get(&c)?) as u64;
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_with_three_letters() {
        let answer = solve("I + BB == ILL");
        let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();
        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn solution_must_have_unique_value_for_each_letter() {
        let answer = solve("A == B");
        assert_eq!(answer, None);
    }

    #[test]
    fn leading_zero_solution_is_invalid() {
        let answer = solve("ACA + DD == BD");
        assert_eq!(answer, None);
    }

    #[test]
    fn puzzle_with_two_digits_final_carry() {
        let answer = solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC");
        let expected = [('A', 9), ('B', 1), ('C', 0)].into_iter().collect();
        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_four_letters() {
        let answer = solve("AS + A == MOM");
        let expected = [('A', 9), ('S', 2), ('M', 1), ('O', 0)]
            .into_iter()
            .collect();
        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_six_letters() {
        let answer = solve("NO + NO + TOO == LATE");
        let expected = [('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]
            .into_iter()
            .collect();
        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_seven_letters() {
        let answer = solve("HE + SEES + THE == LIGHT");
        let expected = [
            ('E', 4),
            ('G', 2),
            ('H', 5),
            ('I', 0),
            ('L', 1),
            ('S', 9),
            ('T', 7),
        ]
            .into_iter()
            .collect();
        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_eight_letters() {
        let answer = solve("SEND + MORE == MONEY");
        let expected = [
            ('S', 9),
            ('E', 5),
            ('N', 6),
            ('D', 7),
            ('M', 1),
            ('O', 0),
            ('R', 8),
            ('Y', 2),
        ]
            .into_iter()
            .collect();
        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_ten_letters() {
        let answer = solve("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE");
        let expected = [
            ('A', 5),
            ('D', 3),
            ('E', 4),
            ('F', 7),
            ('G', 8),
            ('N', 0),
            ('O', 2),
            ('R', 1),
            ('S', 6),
            ('T', 9),
        ]
            .into_iter()
            .collect();
        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_ten_letters_and_199_addends() {
        let answer = solve(
            "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
        );
        let expected = [
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9),
        ]
            .into_iter()
            .collect();
        assert_eq!(answer, Some(expected));
    }

}