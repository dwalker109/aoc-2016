use itertools::Itertools;
use regex::Regex;
fn main() {
    println!("Part 1: {}", part_1(include_str!("../input")));
}

fn part_1(input: &'static str) -> usize {
    let regex =
        Regex::new(r#"(?P<enc_name>.*)-(?P<sector_id>[0-9]+)\[(?P<checksum>[\w]+)\]"#).unwrap();

    regex.captures_iter(input).fold(0usize, |acc, c| {
        let counts = &c["enc_name"].chars().filter(|c| c != &'-').counts();

        let calculated_checksum = counts
            .iter()
            .sorted_by_key(|(&ltr, _)| ltr)
            .sorted_by_key(|(_, &qty)| -(qty as isize))
            .map(|(&ltr, _)| ltr)
            .take(5)
            .join("");

        match calculated_checksum == c["checksum"] {
            true => acc + c["sector_id"].parse::<usize>().unwrap(),
            false => acc,
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let r = super::part_1(include_str!("../input"));
        assert_eq!(r, 158835);
    }
}
