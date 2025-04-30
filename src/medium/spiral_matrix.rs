enum Direction { E, S, W, N, }

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size];
    let (mut val, mut dir, mut row, mut col) = (1, Direction::E, 0, 0);

    while val <= size*size {
        matrix[row][col] = val as u32;
        
        match dir {
            Direction::E => {
                if check_cell(&matrix, row as i32, (col as i32) + 1, size) {
                    col += 1;                    
                } else {
                    dir = Direction::S;
                    row += 1;
                }
            }
            Direction::S => {
                if check_cell(&matrix, (row as i32) + 1, col as i32, size) {
                    row += 1;
                } else {
                    dir = Direction::W;
                    col -= 1;
                }
            }
            Direction::W => {
                if check_cell(&matrix, row as i32, (col as i32) - 1, size) {
                    col -= 1;
                } else {
                    dir = Direction::N;
                    row -= 1;
                }
            }
            Direction::N => {
                if check_cell(&matrix, (row as i32) - 1, col as i32, size) {
                    row -= 1;
                } else {
                    dir = Direction::E;
                    col += 1;
                }
            }
        }
        val += 1;
    }
    matrix
}

fn check_cell(matrix: &[Vec<u32>], i: i32, j: i32, size: usize) -> bool {
    i >= 0 && j >= 0 && 
        (i as usize) < size && (j as usize) < size && 
        matrix[i as usize][j as usize] == 0
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
