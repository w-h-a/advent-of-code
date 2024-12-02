fn main() {
    let input = include_str!("./input2.txt");

    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut left = vec![];

    let mut right = vec![];

    for line in input.lines() {
        let mut two_ids = line.split_whitespace();

        left.push(two_ids.next().unwrap().parse::<i32>().unwrap());

        right.push(two_ids.next().unwrap().parse::<i32>().unwrap());
    }

    let mut result = 0;

    for left_id in &left {
        let mut count = 0;

        for right_id in &right {
            if *right_id == *left_id {
                count += 1;
            }
        }

        result += *left_id * count;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );

        assert_eq!(result, 31);
    }
}
