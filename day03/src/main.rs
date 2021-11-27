use itertools::{self, Itertools};

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input")));
}

fn part_1(input: &'static str) -> u32 {
    input.lines().fold(0u32, |acc, el| {
        let is_triangle = el
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .permutations(3)
            .all(|p| {
                let &a = p.get(0).unwrap();
                let &b = p.get(1).unwrap();
                let &c = p.get(2).unwrap();

                a + b > c
            });

        acc + (is_triangle as u32)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let r = super::part_1(include_str!("../input_test"));
        assert_eq!(r, 917)
    }
}
