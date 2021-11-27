use itertools::Itertools;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input")));
    println!("Part 2: {}", part_2(include_str!("../input")));
}

fn part_1(input: &'static str) -> u32 {
    input
        .lines()
        .map(split_to_u32)
        .fold(0u32, |acc, el| acc + (is_triangle(el) as u32))
}

fn part_2(input: &'static str) -> u32 {
    let col_chunked = split_to_u32(input).chunks(9);

    let reordered = col_chunked.into_iter().flat_map(|c| {
        let v = c.collect_vec();
        let [a1, b1, c1, a2, b2, c2, a3, b3, c3] = <[u32; 9]>::try_from(v).unwrap();
        vec![
            vec![a1, a2, a3].into_iter(),
            vec![b1, b2, b3].into_iter(),
            vec![c1, c2, c3].into_iter(),
        ]
    });

    reordered.fold(0u32, |acc, el| acc + (is_triangle(el) as u32))
}

fn split_to_u32(input: &'static str) -> impl Iterator<Item = u32> {
    input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
}

fn is_triangle(maybe_triangle: impl Iterator<Item = u32>) -> bool {
    maybe_triangle.permutations(3).all(|p| {
        let [a, b, c] = <[u32; 3]>::try_from(p).unwrap();

        a + b > c
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let r = super::part_1(include_str!("../input"));
        assert_eq!(r, 917)
    }

    #[test]
    fn part_2() {
        let r = super::part_2(include_str!("../input"));
        assert_eq!(r, 1649)
    }
}
