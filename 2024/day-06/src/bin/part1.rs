fn main() {
    let input = include_str!("input.txt");

    let output = part1(input);

    dbg!(output);
}

// x, y (col, row)
const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);

fn part1(input: &str) -> i32 {
    let (mut matrix, mut initial_position) = input_to_matrix(input);

    matrix[initial_position.0][initial_position.1] = 'X';

    walk(&mut matrix, &mut initial_position, &UP);

    let mut result = 0;

    for row in matrix {
        for character in row {
            if character == 'X' {
                result += 1;
            }
        }
    }

    return result;
}

// position is y (row), x (col) while direction x (col), y (row)
fn walk(matrix: &mut Vec<Vec<char>>, position: &mut (usize, usize), direction: &(isize, isize)) {    
    let mut next_col = (*position).1 as isize;
    let mut next_row = (*position).0 as isize;

    next_col += (*direction).0;
    next_row += (*direction).1;

    if out_of_bounds((*matrix).len(), (*matrix)[0].len(), &(next_row, next_col)) {
        return;
    }

    if (*matrix)[next_row as usize][next_col as usize] == '#' {
        if (*direction) == UP {
            return walk(matrix, position, &RIGHT);
        } else if (*direction) == RIGHT {
            return walk(matrix, position, &DOWN);
        } else if (*direction) == DOWN {
            return walk(matrix, position, &LEFT);
        } else {
            return walk(matrix, position, &UP);
        }
    }

    (*matrix)[next_row as usize][next_col as usize] = 'X';

    return walk(
        matrix,
        &mut (next_row as usize, next_col as usize),
        direction,
    );
}

// position is y (row), x (col)
fn out_of_bounds(matrix_len: usize, row_len: usize, position: &(isize, isize)) -> bool {
    if (*position).0 < 0 as isize || (*position).0 >= row_len as isize {
        return true;
    }

    if (*position).1 < 0 as isize || (*position).1 >= matrix_len as isize {
        return true;
    }

    return false;
}

fn input_to_matrix(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut matrix = vec![];

    let mut initial_position = (0 as usize, 0 as usize);

    let mut current_row = vec![];

    for char in input.chars() {
        if char == '\n' {
            matrix.push(current_row);
            current_row = vec![];
        } else {
            current_row.push(char);
        }

        if char == '^' {
            // row, col (y, x)
            initial_position = (matrix.len(), current_row.len() - 1)
        }
    }

    matrix.push(current_row);

    return (matrix, initial_position);
}
