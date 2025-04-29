pub fn encode(n: u64) -> String {
    if n == 0 { return String::from("zero"); }

    const UNITS: [(&str, u64); 6] = [
        ("quintillion", 1_000_000_000_000_000_000),
        ("quadrillion", 1_000_000_000_000_000),
        ("trillion", 1_000_000_000_000),
        ("billion", 1_000_000_000),
        ("million", 1_000_000),
        ("thousand", 1_000),
    ];

    let say = |n| match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        _ => unreachable!(),
    };

    let sub100 = |n| match n {
        1..=19 => say(n).into(),
        20..=99 if n % 10 == 0 => say(n).into(),
        21..=99 if n % 10 > 0 => format!("{}-{}", say((n / 10) * 10), say(n % 10)),
        _ => String::new(),
    };

    let sub1000 = |n| match n {
        1..=99 => sub100(n),
        100..=999 => format!("{} {} {}", say(n / 100), say(100), sub100(n % 100)).trim().to_string(),
        _ => String::new(),
    };

    let (mut parts, mut n) = (Vec::new(), n);
    for &(name, unit) in &UNITS {
        if n >= unit {
            let count = n / unit;
            n %= unit;
            if count > 0 {
                parts.push(format!("{} {}", sub1000(count), name));
            }
        }
    }

    if n > 0 {
        parts.push(sub1000(n));
    }

    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let input = 0;
        let output = encode(input);
        let expected = "zero";
        assert_eq!(output, expected);
    }

    #[test]
    fn one() {
        let input = 1;
        let output = encode(input);
        let expected = "one";
        assert_eq!(output, expected);
    }

    #[test]
    fn fourteen() {
        let input = 14;
        let output = encode(input);
        let expected = "fourteen";
        assert_eq!(output, expected);
    }

    #[test]
    fn twenty() {
        let input = 20;
        let output = encode(input);
        let expected = "twenty";
        assert_eq!(output, expected);
    }

    #[test]
    fn twenty_two() {
        let input = 22;
        let output = encode(input);
        let expected = "twenty-two";
        assert_eq!(output, expected);
    }

    #[test]
    fn thirty() {
        let input = 30;
        let output = encode(input);
        let expected = "thirty";
        assert_eq!(output, expected);
    }

    #[test]
    fn ninety_nine() {
        let input = 99;
        let output = encode(input);
        let expected = "ninety-nine";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_hundred() {
        let input = 100;
        let output = encode(input);
        let expected = "one hundred";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_hundred_twenty_three() {
        let input = 123;
        let output = encode(input);
        let expected = "one hundred twenty-three";
        assert_eq!(output, expected);
    }

    #[test]
    fn two_hundred() {
        let input = 200;
        let output = encode(input);
        let expected = "two hundred";
        assert_eq!(output, expected);
    }

    #[test]
    fn nine_hundred_ninety_nine() {
        let input = 999;
        let output = encode(input);
        let expected = "nine hundred ninety-nine";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_thousand() {
        let input = 1_000;
        let output = encode(input);
        let expected = "one thousand";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_thousand_two_hundred_thirty_four() {
        let input = 1_234;
        let output = encode(input);
        let expected = "one thousand two hundred thirty-four";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_million() {
        let input = 1_000_000;
        let output = encode(input);
        let expected = "one million";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_million_two_thousand_three_hundred_forty_five() {
        let input = 1_002_345;
        let output = encode(input);
        let expected = "one million two thousand three hundred forty-five";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_billion() {
        let input = 1_000_000_000;
        let output = encode(input);
        let expected = "one billion";
        assert_eq!(output, expected);
    }

    #[test]
    fn a_big_number() {
        let input = 987_654_321_123;
        let output = encode(input);
        let expected = "nine hundred eighty-seven billion six hundred fifty-four million three hundred twenty-one thousand one hundred twenty-three";
        assert_eq!(output, expected);
    }

    #[test]
    fn max_i64() {
        let input = 9_223_372_036_854_775_807;
        let output = encode(input);
        let expected = "nine quintillion two hundred twenty-three quadrillion three hundred seventy-two trillion thirty-six billion eight hundred fifty-four million seven hundred seventy-five thousand eight hundred seven";
        assert_eq!(output, expected);
    }

    #[test]
    fn max_u64() {
        let input = 18_446_744_073_709_551_615;
        let output = encode(input);
        let expected = "eighteen quintillion four hundred forty-six quadrillion seven hundred forty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen";
        assert_eq!(output, expected);
    }

    #[test]
    fn additonal() {
        assert_eq!(encode(1000000001), "one billion one");
    }
}
