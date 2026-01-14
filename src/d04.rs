#![feature(array_try_from_fn)]

use aoc2016::{ALPHABET_COUNT, AlphabetMap};
use lazy_static::lazy_static;
use regex::Regex;
use std::array::from_fn;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct Room<'s> {
    name: &'s str,
    sector_id: u32,
    checksum: [char; 5],
}

// can't use FromStr because that doesn't allow you to return a type depending on the lifetime
// of the &str
impl<'s> TryFrom<&'s str> for Room<'s> {
    type Error = ();

    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        lazy_static! {
            // language=regexp
            static ref Pat: Regex = Regex::new(r#"([a-z\-]*?)-(\d+)\[([a-z]{5})]"#).unwrap();
        }
        let (_, [name, sector_id, checksum]) = Pat.captures(s).ok_or(())?.extract();
        let mut c = checksum.chars();
        Ok(Room {
            name,
            sector_id: sector_id.parse().map_err(|_| ())?,
            // unwrap is safe here because the regex would have failed already.
            checksum: from_fn(|_| c.next().unwrap()),
        })
    }
}

fn calculate_checksum(name: &str) -> [char; 5] {
    const {
        assert!(ALPHABET_COUNT >= 5);
    }
    let mut counts = AlphabetMap::<usize>::new();
    for c in name.chars() {
        if !c.is_ascii_lowercase() {
            continue;
        }
        counts[c] += 1;
    }
    let mut entries = <[(char, usize); _]>::from(counts);
    entries.sort_by_key(|&(c, count)| (usize::MAX - count, c));
    let mut it = entries.into_iter().map(|(c, _)| c);
    from_fn(|_| it.next().unwrap())
}

pub fn decrypt_string(encrypted: &str, by: u32) -> String {
    encrypted
        .chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                (((((c as u8 - b'a') as u32 + by) % ALPHABET_COUNT as u32) as u8) + b'a') as char
            }
        })
        .collect()
}

impl Room<'_> {
    fn is_valid(&self) -> bool {
        calculate_checksum(self.name) == self.checksum
    }

    fn decrypt_name(&self) -> String {
        decrypt_string(self.name, self.sector_id)
    }
}

fn main() {
    let input = include_str!("d04.txt");
    let valid_rooms: Vec<_> = input
        .lines()
        .map(Room::try_from)
        .map(Result::unwrap)
        .filter(Room::is_valid)
        .collect();
    let sector_sum: u32 = valid_rooms.iter().map(|r| r.sector_id).sum();
    println!("Part1: {}", sector_sum);

    let north_pole_storage = valid_rooms
        .into_iter()
        // .inspect(|r| {
        //     println!("{:?} = {}", r, r.decrypt_name());
        // })
        .find(|r| r.decrypt_name() == "northpole object storage")
        .unwrap();
    println!("Part2: {}", north_pole_storage.sector_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "aaaaa-bbb-z-y-x-123[abxyz]".try_into(),
            Ok(Room {
                name: "aaaaa-bbb-z-y-x",
                sector_id: 123,
                checksum: ['a', 'b', 'x', 'y', 'z']
            })
        )
    }

    #[test]
    fn test_calculate_checksum() {
        assert_eq!(
            calculate_checksum("aaaaa-bbb-z-y-x"),
            ['a', 'b', 'x', 'y', 'z']
        );
        assert_eq!(
            calculate_checksum("a-b-c-d-e-f-g-h"),
            ['a', 'b', 'c', 'd', 'e']
        );
        assert_eq!(
            calculate_checksum("not-a-real-room"),
            ['o', 'a', 'r', 'e', 'l']
        );
        assert_eq!(
            calculate_checksum("totally-real-room"),
            ['l', 'o', 'a', 'r', 't']
        );
    }

    #[test]
    fn test_is_valid() {
        assert_eq!(
            [
                "aaaaa-bbb-z-y-x-123[abxyz]",
                "a-b-c-d-e-f-g-h-987[abcde]",
                "not-a-real-room-404[oarel]",
                "totally-real-room-200[decoy]",
            ]
            .map(|s| Room::try_from(s).unwrap().is_valid()),
            [true, true, true, false]
        );
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            decrypt_string("qzmt-zixmtkozy-ivhz", 343),
            "very encrypted name"
        );
    }
}
