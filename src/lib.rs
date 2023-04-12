use crate::AffineCipherError::NotCoprime;
use itertools::Itertools;

mod my_tests;
mod tests;

const M: i32 = 26;

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
                    Some(int_to_char(((a * char_to_int(c) + b) % M) as u32))
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
    match gcdx(a, b) {
        (1, mmi, _) => Ok(ciphertext
            .chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(int_to_char(((mmi * (char_to_int(c) - b)) % M) as u32))
                } else if c.is_digit(10) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect()),
        _ => Err(NotCoprime(a)),
    }
}

fn char_to_int(char: char) -> i32 {
    char.to_ascii_lowercase() as i32 - 'a' as i32
}

fn int_to_char(input: u32) -> char {
    (input + 'a' as u32) as u8 as char
}

/// Given two input values a and b, finds the GCD of a and b.
fn gcd(a: i32, b: i32) -> i32 {
    return if b == 0 { a } else { gcd(b, a % b) };
}

/// Given two input values a and b, finds the GCD of a and b. Also finds x and y which satisfy
/// the equation ax + by = GCD(a, b).
fn gcdx(a: i32, b: i32) -> (i32, i32, i32) {
    return if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = gcdx(b, a % b);
        (gcd, y, x - ((a / b) * y))
    };
}
