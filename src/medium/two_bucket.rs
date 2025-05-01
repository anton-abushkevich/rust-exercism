use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket { One, Two }

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(capacity1: u8, capacity2: u8, goal: u8, start: &Bucket) -> Option<BucketStats> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let (init_b1, init_b2) = match start {
        Bucket::One => (capacity1, 0),
        Bucket::Two => (0, capacity2),
    };

    queue.push_back((init_b1, init_b2, 1));
    visited.insert((init_b1, init_b2));

    while let Some((b1, b2, moves)) = queue.pop_front() {
        if b1 == goal {
            return Some(BucketStats { moves, goal_bucket: Bucket::One, other_bucket: b2 });
        }
        if b2 == goal {
            return Some(BucketStats { moves, goal_bucket: Bucket::Two, other_bucket: b1 });
        }

        for (nb1, nb2) in [
            (capacity1, b2),
            (b1, capacity2),
            (0, b2),
            (b1, 0),
            (b1 - b1.min(capacity2 - b2), b2 + b1.min(capacity2 - b2)),
            (b1 + b2.min(capacity1 - b1), b2 - b2.min(capacity1 - b1)),
        ] {
            if (nb1 == 0 && nb2 == capacity2 && matches!(start, Bucket::One)) ||
                (nb2 == 0 && nb1 == capacity1 && matches!(start, Bucket::Two)) {
                continue;
            }
            if visited.insert((nb1, nb2)) {
                queue.push_back((nb1, nb2, moves + 1));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_one() {
        let output = solve(3, 5, 1, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 4,
            goal_bucket: Bucket::One,
            other_bucket: 5,
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn measure_using_bucket_one_of_size_3_and_bucket_two_of_size_5_start_with_bucket_two() {
        let output = solve(3, 5, 1, &Bucket::Two);
        let expected = Some(BucketStats {
            moves: 8,
            goal_bucket: Bucket::Two,
            other_bucket: 3,
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_one() {
        let output = solve(7, 11, 2, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 14,
            goal_bucket: Bucket::One,
            other_bucket: 11,
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn measure_using_bucket_one_of_size_7_and_bucket_two_of_size_11_start_with_bucket_two() {
        let output = solve(7, 11, 2, &Bucket::Two);
        let expected = Some(BucketStats {
            moves: 18,
            goal_bucket: Bucket::Two,
            other_bucket: 7,
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn measure_one_step_using_bucket_one_of_size_1_and_bucket_two_of_size_3_start_with_bucket_two() {
        let output = solve(1, 3, 3, &Bucket::Two);
        let expected = Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn measure_using_bucket_one_of_size_2_and_bucket_two_of_size_3_start_with_bucket_one_and_end_with_bucket_two()
    {
        let output = solve(2, 3, 3, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: 2,
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn not_possible_to_reach_the_goal() {
        let output = solve(6, 15, 5, &Bucket::One);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn with_the_same_buckets_but_a_different_goal_then_it_is_possible() {
        let output = solve(6, 15, 9, &Bucket::One);
        let expected = Some(BucketStats {
            moves: 10,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn goal_larger_than_both_buckets_is_impossible() {
        let output = solve(5, 7, 8, &Bucket::One);
        let expected = None;
        assert_eq!(output, expected);
    }
}