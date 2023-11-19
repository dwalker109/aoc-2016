use itertools::Itertools;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> String {
    let w = input.lines().next().unwrap().len();

    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate()
        .fold(vec![Vec::new(); w], |mut acc, (i, c)| {
            acc[i % w].push(c);
            acc
        })
        .iter()
        .map(|v| {
            v.iter()
                .counts()
                .iter()
                .max_by_key(|x| x.1)
                .unwrap()
                .0
                .to_owned()
        })
        .collect()
}

fn part2(input: &'static str) -> String {
    let w = input.lines().next().unwrap().len();

    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate()
        .fold(vec![Vec::new(); w], |mut acc, (i, c)| {
            acc[i % w].push(c);
            acc
        })
        .iter()
        .map(|v| {
            v.iter()
                .counts()
                .iter()
                .min_by_key(|x| x.1)
                .unwrap()
                .0
                .to_owned()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        assert_eq!(super::part1(INPUT), "easter");
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part2(INPUT), "advent");
    }
}
