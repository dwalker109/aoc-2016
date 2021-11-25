use regex::Regex;

fn main() {
    let r1 = part_1(std::include_str!("../input"));
    println!("Part 1: {}", r1);
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

fn part_1(input: &'static str) -> i32 {
    let mut facing = Facing::North;
    let mut coords = Coords(0, 0);

    let regex = Regex::new(r#"([R|L])([1-9]+)"#).unwrap();
    for c in regex.captures_iter(input) {
        let d = c.get(1).unwrap().as_str();
        let n = c.get(2).unwrap().as_str();
        facing = facing.rot(d);
        coords = coords.pos(facing, str::parse::<i32>(n).unwrap());
    }

    coords.manhattan()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let result = super::part_1(std::include_str!("../input_test"));
        assert_eq!(result, 2)
    }
}
