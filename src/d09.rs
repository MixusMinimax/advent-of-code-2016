fn decompress(chars: &[u8]) -> Vec<u8> {
    enum State {
        Normal,
        ParsingLen,
        ParsingRep,
        ParsingPat,
    }
    let mut state = State::Normal;

    let mut out = Vec::new();
    let mut len = 0u32;
    let mut rep = 0u32;
    let mut pat = Vec::new();
    for &c in chars {
        match state {
            State::Normal => {
                if c == b'(' {
                    len = 0;
                    state = State::ParsingLen;
                } else {
                    out.push(c);
                }
            }
            State::ParsingLen => match c {
                b'x' => {
                    rep = 0;
                    state = State::ParsingRep;
                }
                b'0'..=b'9' => {
                    len = len * 10 + (c - b'0') as u32;
                }
                _ => panic!("expected [0-9], got '{c}'"),
            },
            State::ParsingRep => match c {
                b')' => {
                    state = if len * rep == 0 {
                        State::Normal
                    } else {
                        pat.clear();
                        State::ParsingPat
                    }
                }
                b'0'..=b'9' => {
                    rep = rep * 10 + (c - b'0') as u32;
                }
                _ => panic!("expected [0-9], got '{c}'"),
            },
            State::ParsingPat => {
                pat.push(c);
                len -= 1;
                if len == 0 {
                    for _ in 0..rep {
                        out.extend(&pat);
                    }
                    state = State::Normal;
                }
            }
        }
    }
    out
}

fn main() {
    let input = include_str!("d09.txt");
    let decompressed = decompress(input.as_bytes());
    println!("Part1: {}", decompressed.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        assert_eq!(decompress(b"ADVENT"), b"ADVENT");
        assert_eq!(decompress(b"A(1x5)BC"), b"ABBBBBC");
        assert_eq!(decompress(b"(3x3)XYZ"), b"XYZXYZXYZ");
        assert_eq!(decompress(b"A(2x2)BCD(2x2)EFG"), b"ABCBCDEFEFG");
        assert_eq!(decompress(b"(6x1)(1x3)A"), b"(1x3)A");
        assert_eq!(decompress(b"X(8x2)(3x3)ABCY"), b"X(3x3)ABC(3x3)ABCY");
    }
}
