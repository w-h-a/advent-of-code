fn main() {
    let input = include_str!("./input1.txt");

    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut left = vec![];

    let mut right = vec![];

    for line in input.lines() {
        let mut two_ids = line.split_whitespace();

        left.push(two_ids.next().unwrap().parse::<i32>().unwrap());

        right.push(two_ids.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();

    right.sort();

    let result = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );

        assert_eq!(result, 11);
    }
}
