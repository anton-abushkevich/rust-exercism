// Ten green bottles hanging on the wall,
// Ten green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be nine green bottles hanging on the wall.

use std::fmt::Write;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let digit_getter = |d: u32| match d {
        10 => "ten",
        9 => "nine",
        8 => "eight",
        7 => "seven",
        6 => "six",
        5 => "five",
        4 => "four",
        3 => "three",
        2 => "two",
        1 => "one",
        0 => "no",
        _ => ""
    };
    let capitalizer = |s: &str| s[..1].to_uppercase() + &s[1..];
    let pluralizer = |digit: u32| if digit == 1 { "" } else { "s" };

    let end = start_bottles - take_down;
    let mut current_bottles = start_bottles;

    let mut song = String::new();
    while current_bottles > end {
        let bottles  = capitalizer(digit_getter(current_bottles));
        let bottles_minus_one = digit_getter(current_bottles - 1);
        writeln!(song,
            "{} green bottle{} hanging on the wall,\n\
             {} green bottle{} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {} green bottle{} hanging on the wall.\n",
            bottles, pluralizer(current_bottles),
            bottles, pluralizer(current_bottles),
            bottles_minus_one, pluralizer(current_bottles - 1)

        ).unwrap();
        current_bottles -= 1;
    }
    String::from(song.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_generic_verse() {
        assert_eq!(
            recite(10, 1).trim(),
            concat!(
            "Ten green bottles hanging on the wall,\n",
            "Ten green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be nine green bottles hanging on the wall.",
            )
        );
    }
}
