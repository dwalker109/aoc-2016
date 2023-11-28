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
    unexploder(input)
}

fn part2(input: &'static str) -> Answer {
    exploder(input).1
}

fn unexploder(input: &str) -> usize {
    let mut input = input;
    let mut n = 0;

    while !input.is_empty() {
        let (rest_input, (literal, marker)) = next_tokens(input).unwrap();
        input = rest_input;

        n += literal.unwrap_or_default().len();

        if let Some((seq, reps)) = marker {
            let (rest_input, next_seq) = take::<_, _, Error<_>>(seq)(input).unwrap();
            input = rest_input;

            n += reps * next_seq.len();
        }
    }

    n
}

fn exploder(mut input: &str) -> (&str, usize) {
    let (rest_input, (literal, marker)) = next_tokens(input).unwrap();
    input = rest_input;

    let mut n = literal.unwrap_or_default().len();

    if let Some((len, rep)) = marker {
        let (_, inner_n) = exploder(&input[..len]);
        input = &input[len..];
        n += rep * inner_n;
    }

    if !input.is_empty() {
        let (rest_input, inner_n) = exploder(input);
        input = rest_input;
        n += inner_n;
    }

    (input, n)
}

fn next_tokens(input: &str) -> IResult<&str, (Option<&str>, Option<(usize, usize)>)> {
    let (input, (literal, _, marker)) = tuple((
        opt(alphanumeric0),
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

    Ok((input, (literal, marker)))
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
        assert_eq!(super::part2("(3x3)XYZ"), 9);
        assert_eq!(super::part2("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(super::part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            super::part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
