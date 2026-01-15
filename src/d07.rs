use itertools::Itertools;

fn supports_tls(ipv7_addr: &str) -> bool {
    let mut is_inside_brackets = false;
    let mut contains_abba = false;
    for (a, b, c, d) in ipv7_addr.chars().tuple_windows() {
        if a == '[' {
            is_inside_brackets = true;
            continue;
        } else if a == ']' {
            is_inside_brackets = false;
            continue;
        }
        let is_abba = a == d
            && b == c
            && a != b
            && a.is_alphabetic()
            && b.is_alphabetic()
            && c.is_alphabetic()
            && d.is_alphabetic();
        if is_inside_brackets && is_abba {
            return false;
        }
        if !is_inside_brackets && is_abba {
            contains_abba = true;
        }
    }
    contains_abba
}

fn main() {
    let input = include_str!("d07.txt");
    let valid_count = input.lines().filter(|l| supports_tls(l)).count();
    println!("Part1: {}", valid_count);
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
}
