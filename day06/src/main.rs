use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

static INPUT: &str = include_str!("../input");

fn part_1(input: &'static str) -> String {
    let (first_line, _) = input.split_once("\n").unwrap();
    let transposed = first_line
        .chars()
        .enumerate()
        .map(|(n, c)| {
            input
                .chars()
                .skip(n)
                .step_by(first_line.len() + "\n".len())
                .into_group_map()
                .collect_vec()
        })
        .collect::<Vec<_>>();

    println!("{:?}", &transposed);

    todo!();
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(&r, "easter");
    }
}
