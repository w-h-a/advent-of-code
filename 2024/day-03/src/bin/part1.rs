use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
enum Item {
    Mul(u32, u32),
}

fn main() {
    let input = include_str!("./input.txt");

    let output = part1(input);

    dbg!(output.unwrap());
}

fn part1(input: &str) -> miette::Result<u32> {
    let (_, items) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let result: u32 = items
        .iter()
        .map(|ins| match ins {
            Item::Mul(a, b) => *a * *b,
        })
        .sum();

    return Ok(result);
}

fn parse(input: &str) -> IResult<&str, Vec<Item>> {
    return many1(many_till(anychar, lex).map(|(_, ins)| ins))(input);
}

fn lex(input: &str) -> IResult<&str, Item> {
    let (input, _) = tag("mul")(input)?;

    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    return Ok((input, Item::Mul(pair.0, pair.1)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = part1(input);

        assert_eq!(result.unwrap(), 161);
    }
}
