use md5::{Digest, Md5};
use std::io::Write;

#[inline(always)]
fn char_from_half_byte(hex: u8) -> u8 {
    b"0123456789abcdef"[(hex & 0x0f) as usize]
}

fn get_password_1(input: &[u8]) -> String {
    let mut buf = Vec::new();
    let mut result = Vec::new();
    for i in 0u64.. {
        // I do this instead of format! because .clear does not touch the capacity.
        // Therefore, if the resulting string is the same length as before (as is the case almost
        // all the time), there will be no allocations.
        buf.clear();
        buf.extend_from_slice(input);
        write!(&mut buf, "{}", i).unwrap();
        let hash = Md5::digest(&buf);
        if hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xf0) == 0 {
            result.push(char_from_half_byte(hash[2] & 0x0f));
            if result.len() == 8 {
                unsafe {
                    // this is fine because we know that every byte is from /[0-9]|[a-f]/.
                    return String::from_utf8_unchecked(result);
                }
            }
        }
    }
    unreachable!()
}

fn get_password_2(input: &[u8]) -> String {
    let mut buf = Vec::new();
    let mut char_count = 0;
    let mut result = [0u8; 8];
    for i in 0u64.. {
        buf.clear();
        buf.extend_from_slice(input);
        write!(&mut buf, "{}", i).unwrap();
        let hash = Md5::digest(&buf);
        if hash[0] == 0
            && hash[1] == 0
            && (hash[2] & 0xf0) == 0
            && let index = (hash[2] & 0x0f) as usize
            && index < 8
        {
            if result[index] != 0 {
                continue;
            }
            result[index] = char_from_half_byte(hash[3] >> 4);
            char_count += 1;
            if char_count == 8 {
                unsafe {
                    return String::from_utf8_unchecked(Vec::from(result));
                }
            }
        }
    }
    unreachable!();
}

fn main() {
    // let input = b"abc";
    let input = b"ffykfhsq";
    let password = get_password_1(input);
    println!("Part1: {}", password);

    let password = get_password_2(input);
    println!("Part2: {}", password);
}
