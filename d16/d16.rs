fn to_bits(s: &[u8]) -> Vec<bool> {
    s.iter().map(|&c| c == b'1').collect()
}

fn to_chars(bits: &[bool]) -> Vec<u8> {
    bits.iter().map(|&b| if b { b'1' } else { b'0' }).collect()
}

fn transform_data(mut bits: Vec<bool>) -> Vec<bool> {
    let c = bits.len();
    bits.reserve(2 * c + 1);
    bits.push(false);
    for i in 0..c {
        bits.push(!bits[c - i - 1]);
    }
    bits
}

fn fill_drive(start: &[u8], size: usize) -> Vec<bool> {
    let mut v = to_bits(start);
    while v.len() < size {
        v = transform_data(v);
    }
    v.truncate(size);
    v
}

fn checksum_iteration(mut data: Vec<bool>) -> Vec<bool> {
    assert!(data.len().is_multiple_of(2));
    for i in 0..data.len() / 2 {
        data[i] = data[i * 2] == data[i * 2 + 1];
    }
    data.truncate(data.len() / 2);
    data
}

fn calculate_checksum(mut data: Vec<bool>) -> Vec<bool> {
    while data.len().is_multiple_of(2) {
        data = checksum_iteration(data);
    }
    data
}

fn main() {
    let input = b"01000100010010111";
    let chk = to_chars(&calculate_checksum(fill_drive(input, 272)));
    println!("Part1: {}", unsafe { String::from_utf8_unchecked(chk) });

    let chk = to_chars(&calculate_checksum(fill_drive(input, 35651584)));
    println!("Part2: {}", unsafe { String::from_utf8_unchecked(chk) });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn test_transform_data() {
        assert_eq!(transform_data(vec![true]), [true, false, false]);
        assert_eq!(transform_data(vec![false]), [false, false, true]);
        assert_eq!(transform_data(vec![true; 5]), to_bits(b"11111000000"));
    }

    #[test]
    fn test_checksum_iteration() {
        assert_eq!(
            checksum_iteration(to_bits(b"110010110100")),
            to_bits(b"110101")
        );
        assert_eq!(checksum_iteration(to_bits(b"110101")), to_bits(b"100"));
        assert!(catch_unwind(|| checksum_iteration(to_bits(b"100"))).is_err());
    }

    #[test]
    fn test_calculate_checksum() {
        assert_eq!(
            to_chars(&calculate_checksum(to_bits(b"110010110100"))),
            b"100"
        );
    }

    #[test]
    fn test_full() {
        assert_eq!(
            to_chars(&calculate_checksum(fill_drive(b"10000", 20))),
            b"01100"
        );
    }
}
