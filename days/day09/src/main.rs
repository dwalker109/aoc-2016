use nom::{
    bytes::complete::take,
    character::complete::{alphanumeric0, char, digit1, newline},
    combinator::map_res,
    combinator::opt,
    error::Error,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

static INPUT: &str = include_str!("../../../input/day09");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let mut input = input;
    let mut output = String::new();

    while !input.is_empty() {
        let (rest_input, (verbatim, (seq, reps))) = consume_to_marker(input).unwrap();
        input = rest_input;

        output.push_str(verbatim);

        let (rest_input, next_seq) = take::<_, _, Error<_>>(seq)(input).unwrap();
        input = rest_input;

        for _ in 0..reps {
            output.push_str(next_seq);
        }
    }

    output.len()
}

fn part2(_input: &'static str) -> Answer {
    todo!();
}

fn consume_to_marker(input: &str) -> IResult<&str, (&str, (usize, usize))> {
    let (input, (verbatim, _, decompression)) = tuple((
        alphanumeric0,
        opt(newline),
        opt(delimited(
            char('('),
            separated_pair(
                map_res(digit1, str::parse::<usize>),
                char('x'),
                map_res(digit1, str::parse::<usize>),
            ),
            char(')'),
        )),
    ))(input)?;

    Ok((input, (verbatim, decompression.unwrap_or_default())))
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1("ADVENT"), 6);
        assert_eq!(super::part1("A(1x5)BC"), 7);
        assert_eq!(super::part1("(3x3)XYZ"), 9);
        assert_eq!(super::part1("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(super::part1("(6x1)(1x3)A"), 6);
        assert_eq!(super::part1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
