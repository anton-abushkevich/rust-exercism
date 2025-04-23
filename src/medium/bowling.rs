#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}


pub struct BowlingGame(Vec<u16>); // vector of frames

impl BowlingGame {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_completed() { return Err(Error::GameComplete) }
        if pins > 10 { return Err(Error::NotEnoughPinsLeft) }

        let rolls_count = self.0.len() + 1;
        let get = |x| *self.0.get(x - 1).unwrap();

        // tenth frame special case
        if rolls_count == 19 {
            self.0.push(pins);
            return Ok(())
        }
        if rolls_count == 20 {
            let prev_roll = get(rolls_count - 1);
            if prev_roll < 10 && prev_roll + pins > 10 { return Err(Error::NotEnoughPinsLeft) }
            self.0.push(pins);
            return Ok(())
        }
        if rolls_count == 21 {
            let prev_prev_roll = get(rolls_count - 2);
            let prev_roll = get(rolls_count - 1);
            if prev_prev_roll == 10 {
                if prev_roll < 10 && prev_roll + pins > 10 { return Err(Error::NotEnoughPinsLeft) }
            }
            self.0.push(pins);
            return Ok(())
        }

        if pins == 10 && rolls_count % 2 == 1 { // strikes store as 10 on odd indices following zero
            self.0.push(pins);
            self.0.push(0);
        } else if rolls_count % 2 == 0 {
            let prev_roll = get(rolls_count - 1);
            if prev_roll + pins > 10 { return Err(Error::NotEnoughPinsLeft) }
            self.0.push(pins);
        } else {
            self.0.push(pins);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_completed() { return None }

        let get = |x| *self.0.get(x).unwrap();

        let mut score = 0;
        for x in (0..18).step_by(2) {
            let first = get(x);
            let second = get(x + 1);
            score += first + second;
            if first == 10 {
                let third = get(x + 2);
                let forth = get(x + 3);
                score += third + forth;
                if third == 10 {
                    score += if x < 16 { get(x + 4) } else { 0 }; // to handle 9th frame
                }
            } else if first + second == 10 {
                score += get(x + 2);
            }
        }

        // tenth frame score
        let tenth_frame_first = get(18);
        if tenth_frame_first == 10 {
            score += 10 + get(19) + get(20);
        } else {
            let tenth_frame_second = get(19);
            if tenth_frame_first + tenth_frame_second == 10 {
                score += 10 + get(20);
            } else {
                score += tenth_frame_first + tenth_frame_second;
            }
        }
        Some(score)
    }

    pub fn is_completed(&self) -> bool {
        if self.0.len() < 20 { return false }
        if self.0.len() == 21 { return true }

        let tenth_frame_first = *self.0.get(18).unwrap();
        let tenth_frame_second = *self.0.get(19).unwrap();
        tenth_frame_first != 10 && tenth_frame_first + tenth_frame_second != 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_returns_a_result() {
        let mut game = BowlingGame::new();
        assert!(game.roll(0).is_ok());
    }

    #[test]
    fn a_roll_cannot_score_more_than_10_points() {
        let mut game = BowlingGame::new();
        assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
    }

    #[test]
    fn should_be_able_to_score_a_game_with_all_zeros() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }

        assert!(game.score().is_some());
    }

    #[test]
    fn an_unstarted_game_cannot_be_scored() {
        let game = BowlingGame::new();
        assert_eq!(game.score(), None);
    }

    #[test]
    fn an_incomplete_game_cannot_be_scored() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), None);
    }

    #[test]
    fn cannot_roll_if_game_already_has_ten_frames() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }

        assert_eq!(game.roll(0), Err(Error::GameComplete));
    }

    #[test]
    fn twenty_zero_pin_rolls_scores_zero() {
        let mut game = BowlingGame::new();
        for _ in 0..20 {
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), Some(0));
    }

    #[test]
    fn should_be_able_to_score_a_game_with_no_strikes_or_spares() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            let _ = game.roll(3);
            let _ = game.roll(6);
        }

        assert_eq!(game.score(), Some(90));
    }

    #[test]
    fn a_spare_followed_by_zeros_is_worth_ten_points() {
        let mut game = BowlingGame::new();
        let _ = game.roll(6);
        let _ = game.roll(4);
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), Some(10));
    }

    #[test]
    fn points_scored_in_the_roll_after_a_spare_are_counted_twice() {
        let mut game = BowlingGame::new();
        let _ = game.roll(6);
        let _ = game.roll(4);
        let _ = game.roll(3);
        for _ in 0..17 {
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), Some(16));
    }

    #[test]
    fn consecutive_spares_each_get_a_one_roll_bonus() {
        let mut game = BowlingGame::new();
        let _ = game.roll(5);
        let _ = game.roll(5);
        let _ = game.roll(3);
        let _ = game.roll(7);
        let _ = game.roll(4);
        for _ in 0..15 {
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), Some(31));
    }

    #[test]
    fn a_spare_in_the_last_frame_gets_a_one_roll_bonus_that_is_counted_once() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(5);
        let _ = game.roll(5);
        let _ = game.roll(7);
        assert_eq!(game.score(), Some(17));
    }

    #[test]
    fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
        let mut game = BowlingGame::new();
        let _ = game.roll(10);
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), Some(10));
    }

    #[test]
    fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
        let mut game = BowlingGame::new();
        let _ = game.roll(10);
        let _ = game.roll(5);
        let _ = game.roll(3);
        for _ in 0..16 {
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), Some(26));
    }

    #[test]
    fn consecutive_strikes_each_get_the_two_roll_bonus() {
        let mut game = BowlingGame::new();
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(5);
        let _ = game.roll(3);
        for _ in 0..12 {
            let _ = game.roll(0);
        }

        assert_eq!(game.score(), Some(81));
    }

    #[test]
    fn a_strike_in_the_last_frame_earns_a_two_roll_bonus_that_is_counted_once() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        let _ = game.roll(7);
        let _ = game.roll(1);
        assert_eq!(game.score(), Some(18));
    }

    #[test]
    fn rolling_a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        let _ = game.roll(7);
        let _ = game.roll(3);
        assert_eq!(game.score(), Some(20));
    }

    #[test]
    fn strikes_with_the_two_roll_bonus_do_not_get_a_bonus_roll() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        assert_eq!(game.score(), Some(30));
    }

    #[test]
    fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_frame_does_not_get_a_bonus() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(7);
        let _ = game.roll(3);
        let _ = game.roll(10);
        assert_eq!(game.score(), Some(20));
    }

    #[test]
    fn all_strikes_is_a_perfect_game() {
        let mut game = BowlingGame::new();
        for _ in 0..12 {
            let _ = game.roll(10);
        }

        assert_eq!(game.score(), Some(300));
    }

    #[test]
    fn two_rolls_in_a_frame_cannot_score_more_than_10_points() {
        let mut game = BowlingGame::new();
        assert!(game.roll(5).is_ok());
        assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
    }

    #[test]
    fn bonus_roll_after_a_strike_in_the_last_frame_cannot_score_more_than_10_points() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
    }

    #[test]
    fn two_bonus_rolls_after_a_strike_in_the_last_frame_cannot_score_more_than_10_points() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        assert!(game.roll(5).is_ok());
        assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
    }

    #[test]
    fn two_bonus_rolls_after_a_strike_in_the_last_frame_can_score_more_than_10_points_if_one_is_a_strike()
    {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        assert!(game.roll(10).is_ok());
        assert!(game.roll(6).is_ok());
    }

    #[test]
    fn the_second_bonus_rolls_after_a_strike_in_the_last_frame_cannot_be_a_strike_if_the_first_one_is_not_a_strike()
    {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        assert!(game.roll(6).is_ok());
        assert_eq!(game.roll(10), Err(Error::NotEnoughPinsLeft));
    }

    #[test]
    fn second_bonus_roll_after_a_strike_in_the_last_frame_cannot_score_more_than_10_points() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        assert!(game.roll(10).is_ok());
        assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
    }

    #[test]
    fn bonus_rolls_for_a_strike_in_the_last_frame_must_be_rolled_before_score_can_be_calculated() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        assert_eq!(game.score(), None);
    }

    #[test]
    fn both_bonus_rolls_for_a_strike_in_the_last_frame_must_be_rolled_before_score_can_be_calculated() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        let _ = game.roll(10);
        assert_eq!(game.score(), None);
        let _ = game.roll(10);
        assert!(game.score().is_some());
    }

    #[test]
    fn bonus_roll_for_a_spare_in_the_last_frame_must_be_rolled_before_score_can_be_calculated() {
        let mut game = BowlingGame::new();
        for _ in 0..18 {
            let _ = game.roll(0);
        }

        let _ = game.roll(5);
        let _ = game.roll(5);
        assert_eq!(game.score(), None);
        let _ = game.roll(10);
        assert!(game.score().is_some());
    }

    #[test]
    fn cannot_roll_after_bonus_rolls_for_strike() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }

        let _ = game.roll(7);
        let _ = game.roll(3);
        assert!(game.roll(2).is_ok());
        assert_eq!(game.roll(2), Err(Error::GameComplete));
    }

    #[test]
    fn cannot_roll_after_bonus_roll_for_strike() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        let _ = game.roll(3);
        assert!(game.roll(2).is_ok());
        assert_eq!(game.roll(2), Err(Error::GameComplete));
    }

    #[test]
    fn last_two_strikes_followed_by_only_last_bonus_with_non_strike_points() {
        let mut game = BowlingGame::new();
        for _ in 0..16 {
            let _ = game.roll(0);
        }

        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(0);
        let _ = game.roll(1);
        assert_eq!(game.score(), Some(31));
    }

}