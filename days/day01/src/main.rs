use std::{
    cmp::{max, min},
    collections::HashMap,
};

use regex::Regex;

static INPUT: &str = include_str!("../../../input/day01");

fn main() {
    aoc_shared::runner::solve(|| part_1_and_2(INPUT).0, || part_1_and_2(INPUT).1.unwrap())
}

#[derive(Clone, Copy)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    fn rot(self, dir: &str) -> Self {
        match dir {
            "L" => match self {
                Facing::North => Facing::West,
                Facing::East => Facing::North,
                Facing::South => Facing::East,
                Facing::West => Facing::South,
            },
            "R" => match self {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North,
            },
            _ => panic!("Can only rotate L or R!"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coords(i32, i32);

impl Coords {
    fn pos(self, f: Facing, n: i32) -> Self {
        let Coords(x, y) = self;

        match f {
            Facing::North => Coords(x, y + n),
            Facing::East => Coords(x + n, y),
            Facing::South => Coords(x, y - n),
            Facing::West => Coords(x - n, y),
        }
    }

    fn manhattan(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

#[derive(Debug, Clone)]
struct History {
    totals: HashMap<Coords, i32>,
    prev: Vec<Coords>,
    first_dupe: Option<Coords>,
}

impl History {
    fn increment(&mut self, ic: Coords, nc: Coords) -> Coords {
        let Coords(ic_x, ic_y) = ic;
        let Coords(nc_x, nc_y) = nc;

        for x in min(ic_x, nc_x)..=max(ic_x, nc_x) {
            for y in min(ic_y, nc_y)..=max(ic_y, nc_y) {
                if self.prev.contains(&Coords(x, y)) {
                    continue;
                }

                let e = self.totals.entry(Coords(x, y)).or_insert(0);
                *e += 1;

                if *e == 2 && self.first_dupe.is_none() {
                    self.first_dupe = Some(Coords(x, y));
                }
            }
        }

        self.prev = vec![ic, nc];

        nc
    }

    fn first_dupe_manhattan(&self) -> Option<i32> {
        Some(self.first_dupe?.manhattan())
    }
}

fn part_1_and_2(input: &'static str) -> (i32, Option<i32>) {
    let mut facing = Facing::North;
    let mut coords = Coords(0, 0);
    let mut history = History {
        totals: HashMap::new(),
        prev: Vec::new(),
        first_dupe: None,
    };

    let regex = Regex::new(r#"([R|L])([1-9]+)"#).unwrap();
    for c in regex.captures_iter(input) {
        let d = c.get(1).unwrap().as_str();
        let n = c.get(2).unwrap().as_str();
        facing = facing.rot(d);
        coords = history.increment(coords, coords.pos(facing, str::parse::<i32>(n).unwrap()));
    }

    (coords.manhattan(), history.first_dupe_manhattan())
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let (result, _) = super::part_1_and_2(INPUT);
        assert_eq!(result, 8)
    }

    #[test]
    fn part_2() {
        let (_, result) = super::part_1_and_2(INPUT);
        assert_eq!(result.unwrap(), 4)
    }
}
