use crate::AffineCipherError::NotCoprime;
use itertools::Itertools;

mod my_tests;
mod tests;

const M: u32 = 26;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`).
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match gcd(a, M as i32) {
        1 => Ok(plaintext
            .chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(int_to_char((a * char_to_int(c) as i32 + b) as u32 % M))
                } else if c.is_digit(10) {
                    Some(c)
                } else {
                    None
                }
            })
            .chunks(5)
            .into_iter()
            .map(|c| c.collect::<String>())
            .join(" ")),
        _ => Err(NotCoprime(a)),
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`).
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mmi = euclid_extended(a, M as i32, None, None, None, None)?.1;
    Ok(ciphertext
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some(int_to_char((mmi as u32 * (char_to_int(c) - b as u32)) % M))
            } else if c.is_digit(10) {
                Some(c)
            } else {
                None
            }
        })
        .collect())
}

fn char_to_int(char: char) -> u32 {
    char.to_ascii_lowercase() as u32 - 'a' as u32
}

fn int_to_char(input: u32) -> char {
    (input + 'a' as u32) as u8 as char
}

/// Given two input values a and b, finds the GCD of a and b.
fn gcd(a: i32, b: i32) -> i32 {
    let rem = a % b;
    return if rem == 0 { b } else { gcd(b, rem) };
}

/// Given two input values a and b, finds the GCD of a and b as well as the x & y values which
/// make the equation ax + by = GCD(a, b) true. Returns (GCD, x, y)
fn euclid_extended(
    a: i32,
    m: i32,
    x0: Option<i32>,
    x1: Option<i32>,
    y0: Option<i32>,
    y1: Option<i32>,
) -> Result<(i32, i32, i32), AffineCipherError> {
    if m == 0 {
        if a == 1 {
            // TODO: Needs to be original a
            return Err(NotCoprime(a))
        } else {
            return Ok((a, x0.unwrap_or(1), y0.unwrap_or(0)));
        }
    } else {
        let quotient = a / m;
        return euclid_extended(
            m,
            a % m,
            Some(x1.unwrap_or(0)),
            Some(x0.unwrap_or(1) - quotient * x1.unwrap_or(1)),
            Some(y1.unwrap_or(1)),
            Some(y0.unwrap_or(0) - quotient * y1.unwrap_or(1)),
        );
    }
}
