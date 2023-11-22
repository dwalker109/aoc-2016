use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, sequence::separated_pair, IResult,
};

static INPUT: &str = include_str!("../../../input/day08");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT, 50, 6), || part2(INPUT, 50, 6))
}

fn part1(input: &'static str, w: usize, h: usize) -> Answer {
    render(input, w, h).0.len()
}

fn part2(input: &'static str, w: usize, h: usize) -> Answer {
    print!("{}", render(input, w, h));

    0
}

fn render(input: &str, w: usize, h: usize) -> Screen {
    let mut screen = Screen::from((input, w, h));
    let commands = input.lines().map(Op::from);

    for op in commands {
        screen.act(&op);
    }

    screen
}

#[derive(Debug, Default)]
struct Screen(HashSet<(usize, usize)>, usize, usize);

impl From<(&str, usize, usize)> for Screen {
    fn from((_value, w, h): (&str, usize, usize)) -> Self {
        Self(HashSet::with_capacity(w * h), w, h)
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.2 {
            for x in 0..self.1 {
                write!(
                    f,
                    "{}",
                    match self.0.contains(&(x, y)) {
                        true => "#",
                        false => ".",
                    }
                )
                .ok();
            }
            writeln!(f).ok();
        }

        writeln!(f)
    }
}

impl Screen {
    pub fn act(&mut self, op: &Op) {
        match op {
            Op::Rect(width, height) => {
                for x in 0..*width {
                    for y in 0..*height {
                        self.0.insert((x, y));
                    }
                }
            }
            Op::Row(target, n) => {
                let new_row = self
                    .0
                    .iter()
                    .filter_map(|(x, y)| (y == target).then_some(((x + n) % self.1, *y)))
                    .collect::<HashSet<_>>();

                self.0.retain(|(_x, y)| y != target);

                self.0.extend(&mut new_row.iter());
            }
            Op::Col(target, n) => {
                let new_col = self
                    .0
                    .iter()
                    .filter_map(|(x, y)| (x == target).then_some((*x, (y + n) % self.2)))
                    .collect::<HashSet<_>>();

                self.0.retain(|(x, _y)| x != target);

                self.0.extend(&mut new_col.iter());
            }
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
        fn rect(input: &str) -> IResult<&str, Op> {
            let (input, (_, (x, y))) = tuple((
                tag("rect "),
                separated_pair(
                    map_res(digit1, str::parse),
                    char('x'),
                    map_res(digit1, str::parse),
                ),
            ))(input)?;

            Ok((input, Op::Rect(x, y)))
        }

        fn row(input: &str) -> IResult<&str, Op> {
            let (input, (_, (y, n))) = tuple((
                tag("rotate row y="),
                separated_pair(
                    map_res(digit1, str::parse),
                    tag(" by "),
                    map_res(digit1, str::parse),
                ),
            ))(input)?;

            Ok((input, Op::Row(y, n)))
        }

        fn col(input: &str) -> IResult<&str, Op> {
            let (input, (_, (x, n))) = tuple((
                tag("rotate column x="),
                separated_pair(
                    map_res(digit1, str::parse),
                    tag(" by "),
                    map_res(digit1, str::parse),
                ),
            ))(input)?;

            Ok((input, Op::Col(x, n)))
        }

        let (_, op) = alt((rect, row, col))(input).unwrap();

        op
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT, 7, 3), 6);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT, 7, 3), 0);
    }
}
