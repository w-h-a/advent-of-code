fn main() {
    let input = include_str!("./input1.txt");

    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut result = 0;

    for report in input.lines() {
        let levels = report.split_whitespace();

        let mut data = vec![];

        for level in levels {
            data.push(level.parse::<i32>().unwrap());
        }

        for index in 0..data.len() {
            let mut new_data = data.clone();

            new_data.remove(index);

            let mut always_increasing = true;

            let mut always_decreasing = true;

            let mut always_gradual = true;

            let mut left_idx = 0;

            let mut right_idx = 1;

            while right_idx < new_data.len() {
                let left = new_data[left_idx];

                let right = new_data[right_idx];

                let diff = left - right;

                if diff < 0 {
                    always_decreasing = false;
                }

                if diff > 0 {
                    always_increasing = false;
                }

                if diff == 0 {
                    always_decreasing = false;
                    always_increasing = false;
                }

                if diff.abs() > 3 {
                    always_gradual = false;
                }

                left_idx += 1;

                right_idx += 1;
            }

            if always_gradual && (always_decreasing || always_increasing) {
                result += 1;
                break;
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = part1(input);

        assert_eq!(result, 4);
    }
}
