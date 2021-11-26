use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part_1(input));
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

fn solve(input: &'static str, pad: Pad, init: Lookup) -> String {
    let mut curr = init;

    input.lines().fold(String::new(), |acc, el| {
        let line_val = el
            .trim()
            .chars()
            .fold::<Lookup, _>(curr, |acc, el| pad.next(el, acc));

        curr = line_val;

        format!("{}{}", acc, pad.0.get(&line_val).unwrap())
    })
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
}
