// this time around, I'm using panics instead of results, because it's a one-off script.
// results are useful for web-servers or other programs that are expected to resume computation.

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct Marker {
    len: usize,
    rep: usize,
}

/// `it` is expected to have already consumed the opening parenthesis.
/// This avoids the caller having to look ahead before consuming characters.
fn parse_marker(it: impl Iterator<Item = u8>) -> Marker {
    enum State {
        ParsingLen,
        ParsingRep,
    }
    let mut state = State::ParsingLen;
    let mut out = Marker::default();
    for c in it {
        match state {
            State::ParsingLen => match c {
                b'0'..=b'9' => {
                    out.len = out.len * 10 + (c - b'0') as usize;
                }
                b'x' => {
                    state = State::ParsingRep;
                }
                _ => panic!("expected [0-9x], got '{c}'"),
            },
            State::ParsingRep => match c {
                b'0'..=b'9' => {
                    out.rep = out.rep * 10 + (c - b'0') as usize;
                }
                b')' => return out,
                _ => panic!("expected [0-9)], got '{c}'"),
            },
        }
    }
    panic!("missing input");
}

fn decompress<'i>(chars: impl IntoIterator<Item = &'i u8>) -> Vec<u8> {
    enum State {
        Normal,
        ParsingPat,
    }
    let mut state = State::Normal;
    let mut it = chars.into_iter().copied();
    let mut out = Vec::new();
    let mut marker = Marker::default();
    let mut pat = Vec::new();
    while let Some(c) = it.next() {
        match state {
            State::Normal => {
                if c == b'(' {
                    marker = parse_marker(&mut it);
                    pat.clear();
                    state = State::ParsingPat;
                } else {
                    out.push(c);
                }
            }
            State::ParsingPat => {
                pat.push(c);
                marker.len -= 1;
                if marker.len == 0 {
                    for _ in 0..marker.rep {
                        out.extend(&pat);
                    }
                    state = State::Normal;
                }
            }
        }
    }
    out
}

fn decompressed_length<'i>(chars: impl IntoIterator<Item = &'i u8>) -> usize {
    // at the very least, the outermost recursion layer doesn't use vtables.
    // recursion depth is limited by the pattern nesting.
    fn inner<It: Iterator<Item = u8>>(mut it: It) -> usize {
        let mut total_length = 0usize;
        while let Some(c) = it.next() {
            if c == b'(' {
                let marker = parse_marker(&mut it);
                total_length += marker.rep
                    * inner::<Box<dyn Iterator<Item = u8>>>(Box::new(it.by_ref().take(marker.len)));
            } else {
                total_length += 1;
            }
        }
        total_length
    }
    inner(chars.into_iter().copied())
}

fn main() {
    let input = include_str!("d09.txt");

    let decompressed = decompress(input.as_bytes());
    println!("Part1: {}", decompressed.len());

    let len = decompressed_length(input.as_bytes());
    println!("Part2: {}", len);
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

    #[test]
    fn test_decompressed_length() {
        assert_eq!(decompressed_length(b""), 0);
        assert_eq!(decompressed_length(b"asd"), 3);
        assert_eq!(decompressed_length(b"(1x3)asd"), 5);
        assert_eq!(decompressed_length(b"(3x3)XYZ"), b"XYZXYZXYZ".len());
        assert_eq!(
            decompressed_length(b"X(8x2)(3x3)ABCY"),
            b"XABCABCABCABCABCABCY".len()
        );
        assert_eq!(
            decompressed_length(b"(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
        assert_eq!(
            decompressed_length(b"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
