static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> usize {
    todo!()
}

fn part2(input: &'static str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        assert_eq!(super::part1(INPUT), 0);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part2(INPUT), 0);
    }
}
