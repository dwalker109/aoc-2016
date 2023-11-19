use md5::{Digest, Md5};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part_1(INPUT), || part_2(INPUT))
}

fn part_1(input: &'static str) -> String {
    let input = input.lines().next().unwrap().as_bytes();
    let mut password = Vec::with_capacity(8);
    let mut hasher = Md5::new();

    for i in 0.. {
        let needle = &[input, i.to_string().as_bytes()].concat();
        hasher.update(needle);
        let res = hasher.finalize_reset();

        if res[..2] == [0, 0] {
            let next_hex_digit = format!("{:02x}", res[2]);
            let mut maybe_match = next_hex_digit.chars();

            if maybe_match.next().unwrap() == '0' {
                password.push(maybe_match.next().unwrap());
            }
        }

        if password.len() == 8 {
            return password.into_iter().collect();
        }
    }

    panic!();
}

fn part_2(input: &'static str) -> String {
    let input = input.lines().next().unwrap().as_bytes();
    let mut password = vec![None; 8];
    let mut hasher = Md5::new();

    for i in 0.. {
        let needle = &[input, i.to_string().as_bytes()].concat();

        hasher.update(&needle);
        let res = hasher.finalize_reset();

        if res[..2] != [0, 0] {
            continue;
        }

        let next_hex_digits = &format!("{:02x}{:02x}", res[2], res[3]);
        let mut maybe_match = next_hex_digits.chars();

        if maybe_match.next().unwrap() != '0' {
            continue;
        }

        let pos = match maybe_match.next().unwrap().to_string().parse::<u8>() {
            Ok(pos) => pos as usize,
            Err(_) => continue,
        };

        if pos < 8 && password[pos] == None {
            password[pos] = maybe_match.next();

            if !password.contains(&None) {
                return password.into_iter().flatten().collect();
            }
        }
    }

    panic!();
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let r = super::part_1(INPUT);
        assert_eq!(r, "18f47a30");
    }

    #[test]
    fn part_2() {
        let r = super::part_2(INPUT);
        assert_eq!(r, "05ace8e3");
    }
}
