use itertools::repeat_n;
use std::collections::VecDeque;

fn run_game(n: usize) -> (usize, i32) {
    assert_ne!(n, 0);
    let mut elves: Vec<_> = repeat_n(1, n).enumerate().collect();
    while elves.len() > 1 {
        let l = elves.len();
        for i in 0..l {
            if elves[i].1 == 0 {
                continue;
            }
            elves[i].1 += elves[(i + 1) % l].1;
            elves[(i + 1) % l].1 = 0;
        }
        elves.retain(|&p| p.1 != 0);
    }
    elves[0]
}

/// This is too expensive. But by running it with lower numbers, I recognized a pattern.
#[allow(dead_code)]
fn run_game_2(n: usize) -> (usize, i32) {
    assert_ne!(n, 0);
    // let pb = ProgressBar::new(n as u64);
    // when removing elements from a VecDeque, it shifts elements from whichever end is closer.
    // this should be twice as fast on average than a Vec.
    let mut elves: VecDeque<_> = repeat_n(1, n).enumerate().collect();
    let mut cur = 0;
    while elves.len() > 1 {
        let across = (cur + elves.len() / 2) % elves.len();
        elves[cur].1 += elves[across].1;
        // this is pretty inefficient, but we need to remove it so we can know who is sitting across
        elves.remove(across);
        if across > cur {
            cur += 1;
        }
        cur %= elves.len();
        // pb.set_position((n - elves.len()) as u64);
    }
    // pb.finish();
    elves[0]
}

/// This is the pattern I found:
fn fake_game_2_by_pattern(n: usize) -> u32 {
    let mut i = 0u32;
    let mut evens = false;

    // (i) starts at 0 and counts up every number, until i*2+1 meets n.
    // then we continue only in even numbers until we get to n (exclusively).
    // then we reset back to 0.
    //
    // this results in:
    // 002: 0
    // 003: 2
    // 004: 0
    // 005: 1
    // 006: 2
    // 007: 4  <- 3*2 would already meet 7-1
    // 008: 6
    // 009: 8
    // 010: 0
    // 011: 1
    // 012: 2
    // 013: 3
    // ...
    // 017: 7
    // 018: 8
    // 019: 10
    // 020: 12
    // 021: 14
    // ...
    // 026: 24
    // 027: 26
    // 028: 0  <- 28 would already meet 28
    // 029: 1
    // 030: 2
    // 031: 3
    // ...
    // 037: 9
    // 038: 10
    // 039: 11
    // ...
    // 051: 23
    // 052: 24
    // 053: 25
    // 054: 26
    // 055: 28 <- 27*2 would already reach 54
    // 056: 30
    // 057: 32
    // ...
    //
    // I don't understand the math behind it, I just ran the simulation to n=1000,
    // and figured out a pattern.

    for l in 2..n {
        if !evens {
            if (i + 1) * 2 >= l as u32 {
                evens = true;
                i = (i + 2) & !1;
            } else {
                i += 1;
            }
        } else if (i + 2) > l as u32 {
            evens = false;
            i = 0;
        } else {
            i += 2;
        }
    }

    i
}

fn main() {
    println!("Part1: {}", run_game(3018458).0 + 1);
    println!("Part2: {}", fake_game_2_by_pattern(3018458) + 1);

    // for i in 2..100 {
    //     println!("{i:0>3}: {}", run_game_2(i).0);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_game() {
        assert_eq!(run_game(5), (2, 5));
    }

    #[test]
    fn test_run_game_2() {
        assert_eq!(run_game_2(5), (1, 5));
    }
}
