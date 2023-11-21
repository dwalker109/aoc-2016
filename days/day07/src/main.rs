use itertools::Itertools;
use std::collections::HashSet;

static INPUT: &str = include_str!("../../../input/day07");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
        .lines()
        .filter_map(|l| {
            let mut hn_active = false;
            let mut abba = None;

            for (l1, l, r, r1) in l.chars().tuple_windows() {
                match l1 {
                    '[' => hn_active = true,
                    ']' => hn_active = false,
                    _ => match l1 == r1 && l == r && l1 != l && r != r1 {
                        true if hn_active => {
                            abba = None;
                            break;
                        }
                        true if !hn_active => {
                            abba = Some(());
                        }
                        _ => continue,
                    },
                }
            }

            abba
        })
        .count()
}

fn part2(input: &'static str) -> Answer {
    #[derive(Eq, PartialEq)]
    enum Mode {
        Aba,
        Bab,
    }

    input
        .lines()
        .filter_map(|l| {
            let mut mode = Mode::Aba;
            let mut aba = HashSet::new();
            let mut bab = HashSet::new();

            for (l, m, r) in l.chars().tuple_windows() {
                match l {
                    '[' => mode = Mode::Bab,
                    ']' => mode = Mode::Aba,
                    _ => match l == r && l != m {
                        true if mode == Mode::Aba => {
                            aba.insert((l, m, r));
                        }
                        true if mode == Mode::Bab => {
                            bab.insert((l, m, r));
                        }
                        _ => continue,
                    },
                }
            }

            aba.iter()
                .any(|(l, m, _r)| bab.contains(&(*m, *l, *m)))
                .then_some(())
        })
        .count()
}

#[cfg(test)]
mod tests {
    static INPUT_1: &str = include_str!("../input_test_1");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_1), 2);
    }

    static INPUT_2: &str = include_str!("../input_test_2");

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_2), 3);
    }
}
