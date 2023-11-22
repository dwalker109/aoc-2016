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
    Row(usize, usize),
    Col(usize, usize),
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        let tok = value.split_whitespace();
        let action = tok.nth(2).unwrap().as_bytes();
        let subj = action
        let dist = tok.nth(1).unwrap().parse::<usize>().unwrap();

        match op[0] {
            b'x' => Self::Col(op[2], dist),
            b'y' => Self::Row(op[2], dist),
        }
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
