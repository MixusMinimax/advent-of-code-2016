#![feature(linked_list_cursors)]

use std::collections::LinkedList;
use std::str::FromStr;

type S = u32;
type R = (S, S);

fn blacklist_range(mut valid: LinkedList<R>, (bs, be): R) -> LinkedList<R> {
    let mut cursor = valid.cursor_front_mut();
    while let Some((s, e)) = cursor.current() {
        if bs <= *s && be >= *e {
            cursor.remove_current();
            continue;
        }
        if bs <= *s && be < *e && be >= *s {
            *s = be + 1;
        } else if bs > *s && bs < *e && be >= *e {
            *e = bs - 1;
        } else if bs > *s && be < *e {
            let old_e = *e;
            *e = bs - 1;
            cursor.insert_after((be + 1, old_e));
            cursor.move_next();
        }
        cursor.move_next();
    }
    valid
}

fn parse_range(s: &str) -> R {
    let mut it = s.split('-').map(|p| S::from_str(p).unwrap());
    (it.next().unwrap(), it.next().unwrap())
}

fn main() {
    let input = include_str!("d20.txt");

    let valid = input
        .lines()
        .map(parse_range)
        .fold(LinkedList::from([(0, S::MAX)]), blacklist_range);

    println!("Part1: {}", valid.front().unwrap().0);

    let total_count: u32 = valid.iter().copied().map(|(s, e)| e + 1 - s).sum();
    println!("Part2: {}", total_count);
}
