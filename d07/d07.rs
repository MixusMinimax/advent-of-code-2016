use itertools::Itertools;
use std::collections::HashSet;

fn supports_tls(ipv7_addr: &str) -> bool {
    let mut is_inside_brackets = false;
    let mut contains_abba = false;
    for (a, b, c, d) in ipv7_addr.as_bytes().iter().copied().tuple_windows() {
        if a == b'[' {
            is_inside_brackets = true;
            continue;
        } else if a == b']' {
            is_inside_brackets = false;
            continue;
        }
        let is_abba = a == d
            && b == c
            && a != b
            && a.is_ascii_alphabetic()
            && b.is_ascii_alphabetic()
            && c.is_ascii_alphabetic()
            && d.is_ascii_alphabetic();
        if is_inside_brackets && is_abba {
            return false;
        }
        if !is_inside_brackets && is_abba {
            contains_abba = true;
        }
    }
    contains_abba
}

fn supports_ssl(ipv7_addr: &str) -> bool {
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    let mut is_inside_brackets = false;
    for (a, b, c) in ipv7_addr.as_bytes().iter().copied().tuple_windows() {
        if a == b'[' {
            is_inside_brackets = true;
            continue;
        } else if a == b']' {
            is_inside_brackets = false;
            continue;
        }
        if a == c
            && a != b
            && a.is_ascii_alphabetic()
            && b.is_ascii_alphabetic()
            && c.is_ascii_alphabetic()
        {
            if is_inside_brackets {
                babs.insert([a, b]);
            } else {
                abas.insert([a, b]);
            }
        }
    }
    abas.into_iter().any(|[a, b]| babs.contains(&[b, a]))
}

fn main() {
    let input = include_str!("d07.txt");
    let valid_count = input.lines().filter(|l| supports_tls(l)).count();
    println!("Part1: {}", valid_count);

    let valid_count = input.lines().filter(|l| supports_ssl(l)).count();
    println!("Part2: {}", valid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_tls() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_supports_ssl() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }
}
