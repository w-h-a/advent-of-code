fn main() {
    let input = include_str!("./input.txt");

    let output = part1(input);

    dbg!(output);
}

// x, y (col, row)
const UP: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (0, -1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);

fn part1(input: &str) -> u32 {
    let word: Vec<char> = vec!['X', 'M', 'A', 'S'];

    let mut result = 0;

    let matrix = input_to_matrix(input);

    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if (*char) == 'X' {
                for directions in vec![
                    vec![UP, LEFT],
                    vec![UP],
                    vec![UP, RIGHT],
                    vec![RIGHT],
                    vec![DOWN, RIGHT],
                    vec![DOWN],
                    vec![DOWN, LEFT],
                    vec![LEFT],
                ] {
                    if found_word(&word, &matrix, 1, x, y, &directions) {
                        result += 1;
                    }
                }
            }
        }
    }

    return result;
}

fn found_word(
    word: &Vec<char>,
    matrix: &Vec<Vec<char>>,
    pos_in_word: usize,
    x: usize,
    y: usize,
    directions: &Vec<(isize, isize)>,
) -> bool {
    if pos_in_word >= (*word).len() {
        return true;
    }

    let mut next_col = x as isize;
    let mut next_row = y as isize;

    for direction in directions {
        next_col += (*direction).0;
        next_row += (*direction).1;
    }

    if next_col < 0 || next_col >= (*matrix)[x].len() as isize {
        return false;
    }

    if next_row < 0 || next_row >= (*matrix).len() as isize {
        return false;
    }

    if (*matrix)[next_row as usize][next_col as usize] != (*word)[pos_in_word] {
        return false;
    }

    return found_word(
        word,
        matrix,
        pos_in_word + 1,
        next_col as usize,
        next_row as usize,
        directions,
    );
}

fn input_to_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix = vec![];

    let mut current_row = vec![];

    for char in input.chars() {
        if char == '\n' {
            matrix.push(current_row);
            current_row = vec![];
        } else {
            current_row.push(char);
        }
    }

    matrix.push(current_row);

    return matrix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = part1(input);

        assert_eq!(result, 18);
    }
}
