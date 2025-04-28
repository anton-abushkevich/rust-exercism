#[derive(Debug)]
pub struct ChessPosition(u8, u8);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..=7).contains(&rank) && (0..=7).contains(&file) {
            return Some(Self(rank as u8, file as u8))
        }
        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.0 == other.0.0 ||            // on same rank 
            self.0.1 == other.0.1 ||        // on same file
            self.0.0.abs_diff(other.0.0) == self.0.1.abs_diff(other.0.1)   // on same diag 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queen_with_a_valid_position() {
        let chess_position = ChessPosition::new(2, 2);
        assert!(chess_position.is_some());
    }

    #[test]
    fn queen_must_have_positive_row() {
        let chess_position = ChessPosition::new(-2, 2);
        assert!(chess_position.is_none());
    }

    #[test]
    fn queen_must_have_row_on_board() {
        let chess_position = ChessPosition::new(8, 4);
        assert!(chess_position.is_none());
    }

    #[test]
    fn queen_must_have_positive_column() {
        let chess_position = ChessPosition::new(2, -2);
        assert!(chess_position.is_none());
    }

    #[test]
    fn queen_must_have_column_on_board() {
        let chess_position = ChessPosition::new(4, 8);
        assert!(chess_position.is_none());
    }

    #[test]
    fn cannot_attack() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());
        assert!(!white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_same_row() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_same_column() {
        let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_first_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_second_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(3, 1).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_third_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(1, 1).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn can_attack_on_fourth_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(1, 7).unwrap());
        let black_queen = Queen::new(ChessPosition::new(0, 6).unwrap());
        assert!(white_queen.can_attack(&black_queen));
    }

    #[test]
    fn cannot_attack_if_falling_diagonals_are_only_the_same_when_reflected_across_the_longest_falling_diagonal()
    {
        let white_queen = Queen::new(ChessPosition::new(4, 1).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 5).unwrap());
        assert!(!white_queen.can_attack(&black_queen));
    }
}