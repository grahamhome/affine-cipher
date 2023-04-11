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
            }).chunks(5).into_iter().map(|c| c.collect::<String>())
            .join(" ")),
        _ => Err(NotCoprime(a)),
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`).
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match gcd(a, M as i32) {
        1 => Err(NotCoprime(a)),
        _ => Ok(ciphertext
            .chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(int_to_char(char_to_int(c) - b as u32 % M))
                } else if c.is_digit(10) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect()),
    }
}

fn char_to_int(char: char) -> u32 {
    char.to_ascii_lowercase() as u32 - 'a' as u32
}

fn int_to_char(input: u32) -> char {
    (input + 'a' as u32) as u8 as char
}

fn gcd(a: i32, b: i32) -> i32 {
    let rem = a % b;
    return if rem == 0 { b } else { gcd(b, rem) };
}
