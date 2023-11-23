use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0},
    combinator::map,
    number::complete::{be_u64, be_u8},
    sequence::{preceded, separated_pair, Tuple},
    IResult, Parser,
};
use std::collections::HashSet;

static INPUT: &str = include_str!("../../../input/day08");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    todo!();
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

#[derive(Debug, Default)]
struct Screen {
    pixels: HashSet<usize, usize>,
    width: usize,
    height: usize,
}

enum Op {
    Rect(usize, usize),
    Row(usize, usize),
    Col(usize, usize),
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        // fn rect(input: &str) -> IResult<&str, Op> {
        //     map(
        //         (tag("rect "), separated_pair(be_u8, char('x'), be_u8)),
        //         |(_, x, y)| Op::Rect(x, y),
        //     )(input)
        // }

        let (_, op) = alt((
            map(
                (tag("rect "), separated_pair(be_u8, char('x'), be_u8)),
                |(_, x, y)| Op::Rect(x, y),
            ),
            map(
                (
                    tag("rotate row y="),
                    separated_pair(be_u8, tag(" by "), be_u8),
                ),
                |(_, y, d)| Op::Row(y, d),
            ),
            map(
                (
                    tag("rotate column x="),
                    separated_pair(be_u8, tag(" by "), be_u8),
                ),
                |(_, y, d)| Op::Col(y, d),
            ),
        ))(input)
        .unwrap();

        todo!()
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 6);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
