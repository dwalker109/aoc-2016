static INPUT: &str = include_str!("../../../input/day08");

type Answer = aoc_shared::runner::AnswerTbd;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    todo!();
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), super::Answer::default());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
