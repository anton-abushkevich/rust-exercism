use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    use Category::*;
    match category {
        Ones => dice.iter().filter(|&&d| d == 1).sum(),
        Twos => dice.iter().filter(|&&d| d == 2).sum(),
        Threes => dice.iter().filter(|&&d| d == 3).sum(),
        Fours => dice.iter().filter(|&&d| d == 4).sum(),
        Fives => dice.iter().filter(|&&d| d == 5).sum(),
        Sixes => dice.iter().filter(|&&d| d == 6).sum(),
        FullHouse => {
            let freq = freq_count(dice);
            if freq.len() == 2 && freq.values().any(|&count| count == 3) {
                dice.iter().sum()
            } else { 0 }
        }
        FourOfAKind => {
            freq_count(dice).iter().filter(|&(_, &count)| count == 4 || count == 5)
                .map(|(&key, _)| key * 4).sum()
        }
        LittleStraight => {
            let freq = freq_count(dice);
            if freq.len() == 5 && !freq.keys().any(|&key| key == 6) { 30 } else { 0 }
        },
        BigStraight => {
            let freq = freq_count(dice);
            if freq.len() == 5 && !freq.keys().any(|&key| key == 1) { 30 } else { 0 }
        },
        Choice => dice.iter().sum(),
        Yacht => if dice.iter().all(|&d| d == dice[0]) { 50 } else { 0 },
    }
}

fn freq_count(dice: Dice) -> HashMap<u8, u32> {
    dice.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yacht() {
        let expected = 50;
        assert_eq!(score([5, 5, 5, 5, 5], Category::Yacht), expected);
    }

    #[test]
    fn not_yacht() {
        let expected = 0;
        assert_eq!(score([1, 3, 3, 2, 5], Category::Yacht), expected);
    }

    #[test]
    fn ones() {
        let expected = 3;
        assert_eq!(score([1, 1, 1, 3, 5], Category::Ones), expected);
    }

    #[test]
    fn ones_out_of_order() {
        let expected = 3;
        assert_eq!(score([3, 1, 1, 5, 1], Category::Ones), expected);
    }

    #[test]
    fn no_ones() {
        let expected = 0;
        assert_eq!(score([4, 3, 6, 5, 5], Category::Ones), expected);
    }

    #[test]
    fn twos() {
        let expected = 2;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Twos), expected);
    }

    #[test]
    fn fours() {
        let expected = 8;
        assert_eq!(score([1, 4, 1, 4, 1], Category::Fours), expected);
    }

    #[test]
    fn yacht_counted_as_threes() {
        let expected = 15;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Threes), expected);
    }

    #[test]
    fn yacht_of_3s_counted_as_fives() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Fives), expected);
    }

    #[test]
    fn fives() {
        let expected = 10;
        assert_eq!(score([1, 5, 3, 5, 3], Category::Fives), expected);
    }

    #[test]
    fn sixes() {
        let expected = 6;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Sixes), expected);
    }

    #[test]
    fn full_house_three_big_others_diff_ones() {
        let expected = 0;
        assert_eq!(score([1, 2, 4, 4, 4], Category::FullHouse), expected);
    }

    #[test]
    fn full_house_two_small_three_big() {
        let expected = 16;
        assert_eq!(score([2, 2, 4, 4, 4], Category::FullHouse), expected);
    }

    #[test]
    fn full_house_three_small_two_big() {
        let expected = 19;
        assert_eq!(score([5, 3, 3, 5, 3], Category::FullHouse), expected);
    }

    #[test]
    fn two_pair_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 4, 4, 5], Category::FullHouse), expected);
    }

    #[test]
    fn four_of_a_kind_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([1, 4, 4, 4, 4], Category::FullHouse), expected);
    }

    #[test]
    fn yacht_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 2, 2, 2], Category::FullHouse), expected);
    }

    #[test]
    fn four_of_a_kind() {
        let expected = 24;
        assert_eq!(score([6, 6, 4, 6, 6], Category::FourOfAKind), expected);
    }

    #[test]
    fn yacht_can_be_scored_as_four_of_a_kind() {
        let expected = 12;
        assert_eq!(score([3, 3, 3, 3, 3], Category::FourOfAKind), expected);
    }

    #[test]
    fn full_house_is_not_four_of_a_kind() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 5, 5], Category::FourOfAKind), expected);
    }

    #[test]
    fn little_straight() {
        let expected = 30;
        assert_eq!(score([3, 5, 4, 1, 2], Category::LittleStraight), expected);
    }

    #[test]
    fn little_straight_as_big_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 5], Category::BigStraight), expected);
    }

    #[test]
    fn four_in_order_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 2, 3, 4], Category::LittleStraight), expected);
    }

    #[test]
    fn no_pairs_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 6], Category::LittleStraight), expected);
    }

    #[test]
    fn minimum_is_1_maximum_is_5_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 3, 4, 5], Category::LittleStraight), expected);
    }

    #[test]
    fn big_straight() {
        let expected = 30;
        assert_eq!(score([4, 6, 2, 5, 3], Category::BigStraight), expected);
    }

    #[test]
    fn big_straight_as_little_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 2], Category::LittleStraight), expected);
    }

    #[test]
    fn no_pairs_but_not_a_big_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 1], Category::BigStraight), expected);
    }

    #[test]
    fn choice() {
        let expected = 23;
        assert_eq!(score([3, 3, 5, 6, 6], Category::Choice), expected);
    }

    #[test]
    fn yacht_as_choice() {
        let expected = 10;
        assert_eq!(score([2, 2, 2, 2, 2], Category::Choice), expected);
    }
}
