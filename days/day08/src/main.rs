use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::tuple;
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

const fn part1<const WIDTH: usize, const HEIGHT: usize>(input: &'static str) -> Answer {
    let screen = Screen(HashSet::new());
    let commands = input.lines().map(Op::from).collect::<Vec<_>>();
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

#[derive(Debug, Default)]
struct Screen(HashSet<(usize, usize)>);

impl Screen {
    pub fn act<const WIDTH: usize, const HEIGHT: usize>(&mut self, op: &Op) {
        match op {
            Op::Rect(width, height) => {
                for x in 0..*width {
                    for y in 0..*height {
                        self.0.insert((x, y));
                    }
                }
            }
            Op::Row(y, n) => {}
            Op::Col(x, n) => {}
        }
    }
}

enum Op {
    Rect(usize, usize),
    Row(usize, usize),
    Col(usize, usize),
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        let rect = |input: &str| -> IResult<&str, Op> {
            let (input, (_, (x, y))) = tuple((
                tag("rect "),
                separated_pair(
                    map_res(digit1, str::parse),
                    char('x'),
                    map_res(digit1, str::parse),
                ),
            ))(input)?;

            Ok((input, Op::Rect(x, y)))
        };

        let row = |input: &str| -> IResult<&str, Op> {
            let (input, (_, (y, n))) = tuple((
                tag("rotate row y= "),
                separated_pair(
                    map_res(digit1, str::parse),
                    tag(" by "),
                    map_res(digit1, str::parse),
                ),
            ))(input)?;

            Ok((input, Op::Row(y, n)))
        };

        let col = |input: &str| -> IResult<&str, Op> {
            let (input, (_, (x, n))) = tuple((
                tag("rotate column x= "),
                separated_pair(
                    map_res(digit1, str::parse),
                    tag(" by "),
                    map_res(digit1, str::parse),
                ),
            ))(input)?;

            Ok((input, Op::Col(x, n)))
        };

        let (_, op) = alt((rect, row, col))(input).unwrap();

        op
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
