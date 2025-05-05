#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    Wink = 1,
    DoubleBlink = 2,
    CloseYourEyes = 4,
    Jump = 8,
    Reverse = 16,
}

impl Action {
    pub fn actions() -> &'static [Action] {
        use Action::*;
        &[Wink, DoubleBlink, CloseYourEyes, Jump]
    }
    
    pub fn map(&self) -> &'static str {
        use Action::*;
        match self {
            Wink => "wink", 
            DoubleBlink => "double blink",
            CloseYourEyes => "close your eyes", 
            Jump => "jump",
            _ => ""
        }
    }
}

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result: Vec<&'static str> = Action::actions()
        .iter()
        .filter(|&&a| a != Action::Reverse && (n & (a as u8)) != 0)
        .map(|a| a.map())
        .collect();
    
    if (n & Action::Reverse as u8) != 0 {
        result.reverse()
    }
    result
}

pub fn _actions(n: u8) -> Vec<&'static str> {
    let mut handshake: Vec<&'static str> = vec![];
    if n & 1 > 0 { handshake.push("wink") }
    if n & 2 > 0 { handshake.push("double blink") }
    if n & 4 > 0 { handshake.push("close your eyes") }
    if n & 8 > 0 { handshake.push("jump") }
    if n & 16 > 0 { handshake.reverse() }
    handshake
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wink_for_1() {
        assert_eq!(actions(1), vec!["wink"])
    }

    #[test]
    fn double_blink_for_10() {
        assert_eq!(actions(2), vec!["double blink"])
    }

    #[test]
    fn close_your_eyes_for_100() {
        assert_eq!(actions(4), vec!["close your eyes"])
    }

    #[test]
    fn jump_for_1000() {
        assert_eq!(actions(8), vec!["jump"])
    }

    #[test]
    fn combine_two_actions() {
        assert_eq!(actions(3), vec!["wink", "double blink"])
    }

    #[test]
    fn reverse_two_actions() {
        assert_eq!(actions(19), vec!["double blink", "wink"])
    }

    #[test]
    fn reversing_one_action_gives_the_same_action() {
        assert_eq!(actions(24), vec!["jump"])
    }

    #[test]
    fn reversing_no_actions_still_gives_no_actions() {
        assert_eq!(actions(16), Vec::<&'static str>::new())
    }

    #[test]
    fn all_possible_actions() {
        assert_eq!(
            actions(15),
            vec!["wink", "double blink", "close your eyes", "jump"]
        )
    }

    #[test]
    fn reverse_all_possible_actions() {
        assert_eq!(
            actions(31),
            vec!["jump", "close your eyes", "double blink", "wink"]
        )
    }

    #[test]
    fn do_nothing_for_zero() {
        assert_eq!(actions(0), Vec::<&'static str>::new())
    }
}