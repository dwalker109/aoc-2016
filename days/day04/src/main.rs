use itertools::Itertools;
use regex::Regex;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(
        || part_1(&get_real_rooms(INPUT)),
        || part_2(&get_real_rooms(INPUT)),
    )
}

fn part_1(rooms: &[Room]) -> usize {
    rooms.iter().map(|r| r.sector_id).sum()
}

fn part_2(rooms: &[Room]) -> usize {
    for r in rooms {
        let decrypted = r
            .enc_name
            .as_bytes()
            .iter()
            .map(|&c| match c {
                b'-' => b' ',
                _ => {
                    let from_char_code = (c - 97) as u32;
                    let ciphered = (from_char_code + (r.sector_id as u32)) % 26;
                    let char_code = ciphered + 97;

                    char_code as u8
                }
            })
            .collect_vec();

        if decrypted == b"northpole object storage" {
            return r.sector_id;
        }
    }

    panic!("could not find northpole object storage");
}

fn get_real_rooms(input: &'static str) -> Vec<Room> {
    let regex =
        Regex::new(r#"(?P<enc_name>.*)-(?P<sector_id>[0-9]+)\[(?P<checksum>[\w]+)\]"#).unwrap();

    let real_rooms = regex
        .captures_iter(input)
        .filter_map(|c| {
            let counts = &c["enc_name"].chars().filter(|c| c != &'-').counts();

            let calculated_checksum = counts
                .iter()
                .sorted_by_key(|(&ltr, _)| ltr)
                .sorted_by_key(|(_, &qty)| -(qty as isize))
                .map(|(&ltr, _)| ltr)
                .take(5)
                .join("");

            match calculated_checksum == c["checksum"] {
                true => Some(Room {
                    enc_name: c["enc_name"].into(),
                    sector_id: c["sector_id"].parse::<usize>().unwrap(),
                }),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    real_rooms
}

struct Room {
    enc_name: String,
    sector_id: usize,
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part_1() {
        let rooms = super::get_real_rooms(INPUT);
        let r = super::part_1(&rooms);
        assert_eq!(r, 2507);
    }

    #[test]
    fn part_2() {
        let rooms = super::get_real_rooms(INPUT);
        let r = super::part_2(&rooms);
        assert_eq!(r, 993);
    }
}
