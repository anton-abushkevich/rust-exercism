#[allow(clippy::needless_range_loop)] // consistency over microoptimization
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // trivial cases to keep things clean in the loop below
    if size == 0 { return vec![] }
    if size == 1 { return vec![vec![1]] }
    
    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size];
    let (mut left, mut right, mut top, mut bottom) = (0, size - 1, 0, size - 1);
    let mut value = 1;

    while left <= right && top <= bottom {
        (left..=right).for_each(|col| { // go east
            matrix[top][col] = value;
            value += 1;
        });
        top += 1;

        (top..=bottom).for_each(|row| {  // go south
            matrix[row][right] = value;
            value += 1;
        });
        right -= 1;

        (left..=right).rev().for_each(|col| {  // go west
            matrix[bottom][col] = value;
            value += 1;
        });
        bottom -= 1;
        
        (top..=bottom).rev().for_each(|row| { // go north
            matrix[row][left] = value;
            value += 1;
        });
        left += 1;
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_spiral() {
        let input = 0;
        let output = spiral_matrix(input);
        let expected: [[u32; 0]; 0] = [];
        assert_eq!(output, expected);
    }

    #[test]
    fn trivial_spiral() {
        let input = 1;
        let output = spiral_matrix(input);
        let expected: [[u32; 1]; 1] = [[1]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_2() {
        let input = 2;
        let output = spiral_matrix(input);
        let expected: [[u32; 2]; 2] = [[1, 2], [4, 3]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_3() {
        let input = 3;
        let output = spiral_matrix(input);
        let expected: [[u32; 3]; 3] = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_4() {
        let input = 4;
        let output = spiral_matrix(input);
        let expected: [[u32; 4]; 4] = [
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7],
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn spiral_of_size_5() {
        let input = 5;
        let output = spiral_matrix(input);
        let expected: [[u32; 5]; 5] = [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ];
        assert_eq!(output, expected);
    }
}
