use std::io::{self, Read};

use anyhow::Result;

/*

when you see a paren, scan until the next paren,
and then split on 'x' to get the length and num_copies

scan ahead that much, and output that many copies of it

(when you're not hitting a paren, emit literally what you see)

*/

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Ignore whitespace.
    let input = input.trim();
    assert!(!input.chars().any(|c| c.is_ascii_whitespace()));

    let mut decompressed = String::new();
    decompress(&input, &mut decompressed);
    dbg!(decompressed.len());

    Ok(())
}

fn decompress(input: &str, output: &mut String) {
    assert!(input.is_ascii());

    let mut i = 0;
    while i < input.len() {
        let c = get(input, i);

        // Normal character, just emit it.
        if c != '(' {
            output.push(c);
            i += 1;
            continue;
        }

        let j = i + input[i..].find(')').unwrap();
        let (len, num_repeats) = parse_nxm(&input[i + 1..=j - 1]);

        let slice = &input[j + 1..][..len];
        for _ in 0..num_repeats {
            output.push_str(slice);
        }

        i = j + 1 + len;
    }
}

/// # Panics
fn get(s: &str, i: usize) -> char {
    let c = s.as_bytes()[i];
    assert!(c.is_ascii());
    c as char
}

/// Parse a string of the form "123x456" into (123, 456).
/// 
/// # Panics
fn parse_nxm(s: &str) -> (usize, usize) {
    let i = s.find('x').unwrap();
    let n = s[..i].parse().unwrap();
    let m = s[i + 1..].parse().unwrap();
    (n, m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("ADVENT", "ADVENT")]
    #[test_case("A(1x5)BC", "ABBBBBC")]
    #[test_case("(3x3)XYZ", "XYZXYZXYZ")]
    #[test_case("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG")]
    #[test_case("(6x1)(1x3)A", "(1x3)A")]
    #[test_case("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY")]
    fn test_decompress(input: &str, expected: &str) {
        let mut actual = String::new();
        decompress(input, &mut actual);
        assert_eq!(expected, actual);
    }
}
