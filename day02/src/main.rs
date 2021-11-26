use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &'static str) -> String {
    let pad = Pad(HashMap::from([
        (Lookup(0, 0), '1'),
        (Lookup(1, 0), '2'),
        (Lookup(2, 0), '3'),
        (Lookup(0, 1), '4'),
        (Lookup(1, 1), '5'),
        (Lookup(2, 1), '6'),
        (Lookup(0, 2), '7'),
        (Lookup(1, 2), '8'),
        (Lookup(2, 2), '9'),
    ]));

    solve(input, pad, Lookup(2, 1))
}

fn part_2(input: &'static str) -> String {
    let pad = Pad(HashMap::from([
        (Lookup(2, 0), '1'),
        (Lookup(1, 1), '2'),
        (Lookup(2, 1), '3'),
        (Lookup(3, 1), '4'),
        (Lookup(0, 2), '5'),
        (Lookup(1, 2), '6'),
        (Lookup(2, 2), '7'),
        (Lookup(3, 2), '8'),
        (Lookup(4, 2), '9'),
        (Lookup(1, 3), 'A'),
        (Lookup(2, 3), 'B'),
        (Lookup(3, 3), 'C'),
        (Lookup(2, 4), 'D'),
    ]));

    solve(input, pad, Lookup(0, 2))
}

fn solve(input: &'static str, pad: Pad, mut curr: Lookup) -> String {
    input
        .lines()
        .map(|el| {
            el.trim().chars().for_each(|el| curr = pad.next(el, curr));
            pad.0.get(&curr).unwrap()
        })
        .collect::<String>()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Lookup(u8, u8);

struct Pad(HashMap<Lookup, char>);

impl Pad {
    fn next(&self, dir: char, curr: Lookup) -> Lookup {
        let Lookup(x, y) = curr;
        let maybe = match dir.to_ascii_uppercase() {
            'U' => Lookup(x, y.saturating_sub(1)),
            'D' => Lookup(x, y + 1),
            'L' => Lookup(x.saturating_sub(1), y),
            'R' => Lookup(x + 1, y),
            _ => unimplemented!(),
        };

        match self.0.contains_key(&maybe) {
            true => maybe,
            false => curr,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let r = super::part_1(include_str!("../input_test"));
        assert_eq!(r, "1985");
    }

    #[test]
    fn part_2() {
        let r = super::part_2(include_str!("../input_test"));
        assert_eq!(r, "5DB3");
    }
}
