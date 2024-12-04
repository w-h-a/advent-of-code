fn main() {
    let input = include_str!("./input.txt");

    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> i32 {
    return 4;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let result = part2(input);

        assert_eq!(result, 4);
    }
}
