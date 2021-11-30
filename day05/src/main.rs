use md5::{Digest, Md5};

fn main() {
    println!("Part 1: {}", part_1("ojvtpuvg"));
}

fn part_1(input: &'static str) -> String {
    let input = input.as_bytes();
    let mut password = Vec::with_capacity(8);
    let mut hasher = Md5::new();

    for i in 0.. {
        let door_id_and_index = &[input, i.to_string().as_bytes()].concat();
        let needle = String::from_utf8_lossy(door_id_and_index);
        hasher.update(needle.as_bytes());
        let res = hasher.finalize_reset();

        let first_4_hex_as_u8 = <[u8; 2]>::try_from(&res.as_slice()[..2]).unwrap();
        if first_4_hex_as_u8 == [0, 0] {
            let next_2_hex_as_u8 = &res.as_slice()[2];
            let next_2_hex_formatted = format!("{:02x}", next_2_hex_as_u8);
            let mut maybe_match = next_2_hex_formatted.chars();
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

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let r = super::part_1("abc");
        assert_eq!(r, "18f47a30");
    }
}
