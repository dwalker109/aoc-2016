use md5::{Digest, Md5};
use std::fmt::Write;
use std::thread;

fn main() {
    let h1 = thread::spawn(move || part_1("ojvtpuvg"));
    let h2 = thread::spawn(move || part_2("ojvtpuvg"));

    println!("Part 1: {}", h1.join().unwrap());
    println!("Part 2: {}", h2.join().unwrap());
}

fn part_1(input: &'static str) -> String {
    let input = input.as_bytes();
    let mut password = Vec::with_capacity(8);
    let mut hasher = Md5::new();

    for i in 0.. {
        let res = next_hash(&mut hasher, &input, &i);

        if res[..2] != [0, 0] {
            continue;
        }
        if res[2] >= 16 {
            continue;
        }

        password.push(std::char::from_digit(res[2] as u32, 16).unwrap());

        if password.len() == 8 {
            return password.into_iter().collect();
        }
    }

    panic!();
}

fn part_2(input: &'static str) -> String {
    let input = input.as_bytes();
    let mut password = vec![None; 8];
    let mut next_hex_digits = String::with_capacity(4);
    let mut hasher = Md5::new();

    for i in 0.. {
        let res = next_hash(&mut hasher, &input, &i);

        if res[..2] != [0, 0] {
            continue;
        }
        if res[2] >= 16 {
            continue;
        }

        next_hex_digits.clear();
        write!(next_hex_digits, "{:x}{:02x}", res[2], res[3]).unwrap();
        let mut maybe_match = next_hex_digits.chars();

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

fn next_hash(hasher: &mut Md5, input: &[u8], i: &usize) -> [u8; 16] {
    let needle = &[input, i.to_string().as_bytes()].concat();
    hasher.update(needle);

    hasher.finalize_reset().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let r = super::part_1("ojvtpuvg");
        assert_eq!(r, "4543c154");
    }

    #[test]
    fn part_2() {
        let r = super::part_2("ojvtpuvg");
        assert_eq!(r, "1050cbbd");
    }
}
